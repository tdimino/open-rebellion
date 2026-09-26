
void __fastcall FUN_005faaf0(int param_1)

{
  int *this;
  int iVar1;
  int iVar2;
  
  this = (int *)(param_1 + 0x1c);
  iVar2 = *(int *)(*(int *)(param_1 + 0x1c) + 4);
  iVar1 = iVar2 + 0x1c + param_1;
  if (*(int *)(iVar2 + 0x50 + param_1) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)(iVar1 + 0x38));
  }
  *(undefined4 *)(iVar1 + 8) = 0;
  if (*(int *)(iVar1 + 0x34) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)(iVar1 + 0x38));
  }
  FUN_00616900(this,0);
  FUN_005f4d00(this,param_1 + 4);
  FUN_005f4d00(this,param_1 + 8);
  FUN_005f4d00(this,param_1 + 0xc);
  FUN_005f4d00(this,param_1 + 0x10);
  FUN_005f4d00(this,param_1 + 0x14);
  FUN_005f4d00(this,param_1 + 0x18);
  FUN_00615680(*(int *)(*(int *)(*this + 4) + 4 + (int)this));
  return;
}

