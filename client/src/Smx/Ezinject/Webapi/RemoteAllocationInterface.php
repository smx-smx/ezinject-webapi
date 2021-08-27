<?php
namespace Smx\Ezinject\Webapi;

/**
 * @author Stefano Moioli
 */
interface RemoteAllocationInterface {
	public function malloc(int $size);
	public function free(int $handle);
}