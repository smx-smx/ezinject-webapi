<?php
namespace Smx\Ezinject\Webapi;

/**
 * @author Stefano Moioli
 */
class RemoteAllocation {
	private RemoteAllocationInterface $alloc;
	private int $ptr;

	public function __construct(RemoteAllocationInterface $alloc, int $ptr){
		$this->alloc = $alloc;
		$this->ptr = $ptr;
	}

	public function getPointer(){
		return $this->ptr;
	}

	public function free(){
		$this->alloc->free($this->ptr);
	}

	public static function sized(RemoteProcess $rproc, int $size){
		$alloc = $rproc->getAllocationApi();
		$ptr = $alloc->malloc($size);
		return new self($alloc, $ptr);
	}

	public static function fromData(RemoteProcess $rproc, string $data){
		$dataSize = strlen($data);
		$alloc = $rproc->getAllocationApi();
		$ptr = $alloc->malloc($dataSize);
		
		$rproc->writeMemory($ptr, $data);
		return new self($alloc, $ptr);
	}
}