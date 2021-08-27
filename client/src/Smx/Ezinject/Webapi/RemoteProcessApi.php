<?php
namespace Smx\Ezinject\Webapi;

/**
 * @author Stefano Moioli
 */
class RemoteProcessApi {
	private string $base_url;
	private bool $debug = false;
	
	public function __construct(string $base_url){
		$this->base_url = $base_url;
	}

	public function getBaseUrl(){
		return $this->base_url;
	}

	private function debug(string $str){
		if($this->debug){
			fwrite(STDERR, "[DEBUG] {$str}\n");
		}
	}

	public function setDebugEnabled(bool $enable){
		$this->debug = $enable;
	}

	private function build_url(string $endpoint, array $params = array()){
		return "{$this->base_url}/{$endpoint}?" . http_build_query($params);
	}

	private function http_get(string $url){
		$this->debug(" GET: {$url}");
		return file_get_contents($url);
	}
	private function http_post(string $url, string $content){
		$this->debug("POST: {$content}");
		$ctx = stream_context_create([
			'http' => [
				'method' => 'POST',
				'header' => 'Content-Type: application/json',
				'content' => $content
			]
		]);
		return file_get_contents($url, false, $ctx);
	}

	public function cfg_info(){
		$url = $this->build_url('cfg');
		return $this->http_get($url);
	}

	public function dlopen(?string $name){
		if($name === null){
			$url = $this->build_url('dlopen/self');
		} else {
			$url = $this->build_url('dlopen', [
				'library' => $name
			]);
		}
		return $this->http_get($url);
	}

	public function dlsym(string $handle, string $sym){
		$url = $this->build_url('dlsym', [
			'handle' => $handle,
			'sym' => $sym
		]);
		return $this->http_get($url);
	}

	public function mem_read(string $addr, string $size, string $format = 'hex'){
		$url = $this->build_url('peek', [
			'addr' => $addr,
			'size' => $size,
			'format' => $format
		]);
		return $this->http_get($url);
	}
	
	public function mem_write(string $addr, string $data){
		$url = $this->build_url('poke', [
			'addr' => $addr
		]);
		return $this->http_post($url, $data);
	}

	public function call(?int $abi, string ...$args){
		$url = $this->build_url('call');
		$params = [
			'fptr' => $args[0],
			'args' => array_slice($args, 1),
		];
		if($abi !== null){
			$params['abi'] = $abi;
		}
		$call_req = json_encode($params);
		return $this->http_post($url, $call_req);
	}
}