
bool __cdecl FUN_0053f760(int *param_1,int param_2,uint param_3)

{
  int *this;
  int iVar1;
  uint uVar2;
  bool bVar3;
  undefined3 extraout_var;
  int *this_00;
  bool bVar4;
  uint auStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uVar2 = param_3;
  iVar1 = param_2;
  this = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006469c0;
  pvStack_c = ExceptionList;
  bVar4 = param_1 != (int *)0x0;
  ExceptionList = &pvStack_c;
  if (bVar4) {
    ExceptionList = &pvStack_c;
    FUN_005f5600(param_1,*(undefined4 *)(param_2 + 0x18));
    (**(code **)(*this + 0x28))(*(undefined4 *)(iVar1 + 0x2c));
    FUN_00540650(this,uVar2);
    FUN_00540630(this,2);
  }
  this_00 = (int *)0x0;
  if (!bVar4) {
    ExceptionList = pvStack_c;
    return false;
  }
  if (this[6] == 3) {
    auStack_14[0] = 0xf1;
    auStack_14[1] = 0xf2;
    uStack_4 = 0;
    this_00 = FUN_0053f030(auStack_14,uVar2,1);
  }
  else {
    bVar3 = FUN_005406d0(this);
    if (CONCAT31(extraout_var,bVar3) == 0) goto LAB_0053f846;
    auStack_14[0] = 0xf2;
    auStack_14[1] = 0xf3;
    uStack_4 = 1;
    this_00 = FUN_0053f030(auStack_14,uVar2,1);
  }
  uStack_4 = 0xffffffff;
  FUN_00619730();
LAB_0053f846:
  if (this_00 != (int *)0x0) {
    FUN_0053a4f0(this_00,this,(int *)&param_1);
  }
  ExceptionList = pvStack_c;
  return bVar4;
}

