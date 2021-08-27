<?php
namespace Smx\Ezinject\Webapi;

/**
 * @author Stefano Moioli
 */
class RemoteLibrary {
	private RemoteProcessApi $rproc;
	private string $handle;
	
	public function __construct(RemoteProcessApi $rproc, string $handle){
		$this->rproc = $rproc;
		$this->handle = $handle;
	}

	public function getSymbol(string $name, ?int $abi = null){
		$fptr = $this->rproc->dlsym($this->handle, $name);
		return new RemoteFunction($this->rproc, $fptr, $abi);
	}
}