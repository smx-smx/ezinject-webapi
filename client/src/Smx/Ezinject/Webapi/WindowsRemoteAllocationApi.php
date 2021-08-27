<?php
namespace Smx\Ezinject\Webapi;

/**
 * @author Stefano Moioli
 */
class WindowsRemoteAllocationApi implements RemoteAllocationInterface {
	private RemoteFunction $malloc;
	private RemoteFunction $free;

	private int $processHeap;

	public function malloc(int $size){
		return $this->malloc->invoke($this->processHeap, 0, $size);
	}

	public function free(int $handle){
		return $this->free->invoke($this->processHeap, 0, $handle);
	}

	public function __construct(RemoteProcess $rproc){
		$kernel32 = $rproc->loadLibrary('kernel32.dll');

		$getProcessHeap = $kernel32->getSymbol('GetProcessHeap');
		$this->processHeap = $getProcessHeap->invoke();

		$this->malloc = $kernel32->getSymbol('HeapAlloc');
		$this->free = $kernel32->getSymbol('HeapFree');
	}
}