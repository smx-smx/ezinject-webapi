<?php
namespace Smx\Ezinject\Webapi;

use InvalidArgumentException;

/**
 * @author Stefano Moioli
 */
class RemoteProcess {
	private RemoteProcessApi $rproc;
	private RemoteAllocationInterface $alloc;
	
	public function __construct(string $base_url){
		$this->rproc = new RemoteProcessApi($base_url);

		$ident = $this->rproc->cfg_info();
		switch($ident){
			case "unix": $this->alloc = new UnixRemoteAllocationApi($this); break;
			case "windows": $this->alloc = new WindowsRemoteAllocationApi($this); break;
			default: throw new InvalidArgumentException("invalid platform {$ident}");
		}
	}

	public function malloc(int $size){
		return $this->alloc->malloc($size);
	}

	public function free(int $handle){
		return $this->alloc->free($handle);
	}

	public function getAllocationApi(){
		return $this->alloc;
	}

	public function setDebugEnabled(bool $enable){
		$this->rproc->setDebugEnabled($enable);
	}

	public function readMemory(int $addr, int $size){
		$hex_addr = sprintf("0x%x", $addr);
		$hex_size = sprintf("0x%x", $size);
		return $this->rproc->mem_read($hex_addr, $hex_size, 'bin');
	}

	public function writeMemory(int $addr, string $data){
		$hex_addr = sprintf("0x%x", $addr);
		return $this->rproc->mem_write($hex_addr, $data);
	}

	public function loadLibrary(?string $name){
		$handle = $this->rproc->dlopen($name);
		return new RemoteLibrary($this->rproc, $handle);
	}
}