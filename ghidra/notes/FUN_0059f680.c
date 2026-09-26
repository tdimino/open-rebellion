
void __thiscall FUN_0059f680(void *this,int param_1)

{
  undefined4 uVar1;
  int iVar2;
  uint local_10 [4];
  
  if (param_1 != 0) {
    iVar2 = FUN_005c7150(param_1);
    if (-1 < iVar2) {
      iVar2 = FUN_005c7150(param_1);
      if (iVar2 < 4) {
        uVar1 = *(undefined4 *)(param_1 + 0x18);
        iVar2 = FUN_005c7150(param_1);
        *(undefined4 *)((int)this + iVar2 * 4 + 0x154) = uVar1;
        FUN_005a0c20(this,local_10);
        FUN_005c0e40((int *)local_10);
      }
    }
  }
  return;
}

