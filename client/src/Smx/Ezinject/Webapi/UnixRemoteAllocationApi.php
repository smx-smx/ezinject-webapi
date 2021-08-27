<?php
namespace Smx\Ezinject\Webapi;

/**
 * @author Stefano Moioli
 */
class UnixRemoteAllocationApi implements RemoteAllocationInterface {
	private RemoteFunction $malloc;
	private RemoteFunction $free;

	public function malloc(int $size){
		return $this->malloc->invoke($size);
	}

	public function free(int $handle){
		return $this->free->invoke($handle);
	}

	public function __construct(RemoteProcess $rproc){
		$self = $rproc->loadLibrary(null);
		$this->malloc = $self->getSymbol('malloc');
		$this->free = $self->getSymbol('free');
	}
}
