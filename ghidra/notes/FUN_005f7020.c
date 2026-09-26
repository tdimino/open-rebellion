
undefined4 __thiscall FUN_005f7020(void *this,undefined4 param_1)

{
  undefined1 *puVar1;
  int iVar2;
  HANDLE hThread;
  
  puVar1 = *(undefined1 **)((int)this + 0x1a8);
  if ((puVar1 != (undefined1 *)0x0) && (iVar2 = *(int *)((int)this + 0x1ac), iVar2 != 0)) {
    *(undefined4 *)((int)this + 0x1a4) = param_1;
    _SmackToBuffer_28(iVar2,(uint)(*(int *)(puVar1 + 0x10) - *(int *)(iVar2 + 4)) >> 1,
                      (uint)(*(int *)(puVar1 + 0x14) - *(int *)(iVar2 + 8)) >> 1,
                      *(int *)(puVar1 + 0x10),*(int *)(puVar1 + 0x14),
                      *(undefined4 *)(puVar1 + 0x448),*puVar1);
    if (*(HANDLE *)((int)this + 0x1b4) != (HANDLE)0x0) {
      TerminateThread(*(HANDLE *)((int)this + 0x1b4),0);
      *(undefined4 *)((int)this + 0x1b4) = 0;
    }
    hThread = CreateThread((LPSECURITY_ATTRIBUTES)0x0,0,lpStartAddress_005f73f0,this,0,
                           (LPDWORD)((int)this + 0x1b0));
    *(HANDLE *)((int)this + 0x1b4) = hThread;
    if (hThread != (HANDLE)0x0) {
      SetThreadPriority(hThread,1);
    }
  }
  return 0;
}

