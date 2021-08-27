<?php
require_once __DIR__ . '/../vendor/autoload.php';

use Smx\Ezinject\Webapi\RemoteAllocation;
use Smx\Ezinject\Webapi\RemoteProcess;

$rproc = new RemoteProcess("http://127.0.0.1:8000/api/v1");
$rproc->setDebugEnabled(true);

$user32 = $rproc->loadLibrary('user32.dll');
$msgbox = $user32->getSymbol('MessageBoxA');

$kernel32 = $rproc->loadLibrary('kernel32.dll');
$allocConsole = $kernel32->getSymbol('AllocConsole');
$allocConsole->invoke();

$lpcText = RemoteAllocation::fromData($rproc, "Hello World!\0");
$lpcTitle = RemoteAllocation::fromData($rproc, "Greetings!\0");

$msgbox->invoke(0, $lpcText->getPointer(), $lpcTitle->getPointer(), 0);

$lpcText->free();
$lpcTitle->free();