
void __fastcall FUN_005d03f0(int param_1)

{
  void *this;
  uint uVar1;
  int iVar2;
  
  uVar1 = *(uint *)(param_1 + 0x80);
  if ((uVar1 < 9) && (*(float *)(param_1 + 0x30) <= *(float *)(param_1 + 0x38 + uVar1 * 4))) {
    uVar1 = *(uint *)(param_1 + 0x5c + uVar1 * 4);
    if (uVar1 != 0x13d) {
      iVar2 = 1;
      this = (void *)FUN_005ba8b0();
      FUN_005ba980(this,uVar1,iVar2);
    }
    *(int *)(param_1 + 0x80) = *(int *)(param_1 + 0x80) + 1;
  }
  return;
}

