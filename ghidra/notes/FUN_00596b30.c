
undefined4 __cdecl FUN_00596b30(int param_1)

{
  void *this;
  int iVar1;
  uint uVar2;
  
  if ((param_1 != 0) && (uVar2 = *(uint *)(param_1 + 0x178), uVar2 != 0)) {
    this = (void *)FUN_00596bd0();
    iVar1 = FUN_005a6480(this,uVar2);
    if (iVar1 != 0) {
      return *(undefined4 *)(iVar1 + 0x38);
    }
  }
  return 0;
}

