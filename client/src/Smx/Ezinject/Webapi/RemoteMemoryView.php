<?php
namespace Smx\Ezinject\Webapi;

use ArrayAccess;

/**
 * @author Stefano Moioli
 */
class RemoteMemoryView implements ArrayAccess {
	private RemoteProcess $rproc;
	private int $baseAddr;
	
	public function __construct(RemoteProcess $rproc, int $baseAddr){
		$this->rproc = $rproc;
		$this->baseAddr = $baseAddr;
	}

	function offsetGet($offset){
		return $this->rproc->readMemory($this->baseAddr + intval($offset), 1);
	}

	function offsetSet($offset, $value){
		$this->rproc->writeMemory($this->baseAddr + intval($offset), $value);
	}

	function offsetExists($offset){
		return true;
	}

	function offsetUnset($offset){}
}