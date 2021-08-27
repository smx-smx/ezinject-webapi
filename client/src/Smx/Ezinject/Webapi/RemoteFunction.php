<?php
namespace Smx\Ezinject\Webapi;

/**
 * @author Stefano Moioli
 */
class RemoteFunction {
	private RemoteProcessApi $rproc;
	private string $fptr;
	private ?int $abi;
	
	public function __construct(RemoteProcessApi $rproc, string $fptr, ?int $abi = null){
		$this->rproc = $rproc;
		$this->fptr = $fptr;
		$this->abi = $abi;
	}

	/**
	 * @param mixed $arg
	 */
	private static function marshal($arg){
		if(is_null($arg)){
			return "0x0";
		}

		if($arg instanceof RemoteAllocation){
			return sprintf("0x%x", $arg->getPointer());
		}

		if(is_bool($arg)){
			$bool = boolval($arg);
			return ($bool) ? '0x1' : '0x0';
		}

		if(is_int($arg)){
			return sprintf("0x%x", $arg);
		}

		if(is_float($arg)){
			$fval = FFI::new('float');
			$fval->cdata = $arg;
			$dval = FFI::cast('uint32_t', $fval);
			return sprintf("0x%x", $dval->cdata);
		}

		if(is_double($arg)){
			$fval = FFI::new('double');
			$fval->cdata = $arg;
			$dval = FFI::cast('uint64_t', $fval);
			return sprintf("0x%x", $dval->cdata);
		}

		return $arg;
	}

	/**
	 * @param mixed[] $arg
	 */
	public function invoke(...$args){
		$values = [];
		foreach($args as $arg){
			$values[]= self::marshal($arg);
		}
		return hexdec( $this->rproc->call($this->abi, $this->fptr, ...$values) );
	}
}
