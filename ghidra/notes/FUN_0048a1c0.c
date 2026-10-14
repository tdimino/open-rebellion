
undefined4 __thiscall FUN_0048a1c0(void *this,uint param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  iVar1 = FUN_005f5500((void *)((int)this + 0x44),param_1);
  if (iVar1 != 0) {
    return *(undefined4 *)(iVar1 + 0x1c);
  }
  uVar2 = FUN_005f4960((undefined4 *)((int)this + 0xc));
  return uVar2;
}

