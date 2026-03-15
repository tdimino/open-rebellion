
void __fastcall FUN_00415ec0(int param_1)

{
  bool bVar1;
  undefined4 uVar2;
  int iVar3;
  undefined3 extraout_var;
  
  if (*(int *)(param_1 + 0xe4) == 2) {
    if (*(int *)(param_1 + 0xdc) != 8) {
      return;
    }
    uVar2 = FUN_0041cdb0();
    FUN_0041d310(uVar2);
    if (*(int *)(*(int *)(param_1 + 0xb8) + 0xac) != 0) {
      return;
    }
    iVar3 = FUN_0041cf80();
    if (((iVar3 == 0) && (bVar1 = FUN_0041dc10(), CONCAT31(extraout_var,bVar1) != 0)) ||
       (iVar3 = FUN_0041dc30(), iVar3 != 0)) {
      FUN_00401200(0,0x1206);
      return;
    }
  }
  else if (*(int *)(param_1 + 0xe4) == 1) {
    uVar2 = FUN_0041cdb0();
    FUN_0041d310(uVar2);
  }
  return;
}

