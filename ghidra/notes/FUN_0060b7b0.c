
void * __thiscall FUN_0060b7b0(void *this,uint param_1,uint param_2,HMODULE param_3)

{
  HRSRC pHVar1;
  HGLOBAL pvVar2;
  undefined1 *puVar3;
  
  pHVar1 = FindResourceA(param_3,(LPCSTR)(param_1 & 0xffff),(LPCSTR)0xa);
  pvVar2 = LoadResource(param_3,pHVar1);
  puVar3 = LockResource(pvVar2);
  if (puVar3 == (undefined1 *)0x0) {
    *(undefined1 *)((int)this + 4) = 0;
  }
  else {
    *(undefined1 *)((int)this + 4) = *puVar3;
  }
  FreeResource(pvVar2);
  pHVar1 = FindResourceA(param_3,(LPCSTR)(param_2 & 0xffff),(LPCSTR)0xa);
  pvVar2 = LoadResource(param_3,pHVar1);
  puVar3 = LockResource(pvVar2);
  if (puVar3 == (undefined1 *)0x0) {
    *(undefined1 *)((int)this + 5) = 0;
  }
  else {
    *(undefined1 *)((int)this + 5) = *puVar3;
  }
  FreeResource(pvVar2);
  *(HMODULE *)((int)this + 8) = param_3;
  *(undefined4 *)this = 0;
  return this;
}

