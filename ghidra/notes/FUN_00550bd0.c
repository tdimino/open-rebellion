
uint FUN_00550bd0(int param_1,int param_2,int param_3)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  void *this;
  undefined4 local_44 [7];
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648ad0;
  local_c = ExceptionList;
  uVar3 = 1;
  iVar1 = 0;
  this = (void *)(param_1 + 4);
  iVar2 = param_2 + 4;
  ExceptionList = &local_c;
  do {
    if (8 < iVar1) {
      ExceptionList = local_c;
      return uVar3;
    }
    FUN_0054fa40(local_28);
    local_4 = 0;
    FUN_0054f8d0(local_44,(int)this);
    local_4 = CONCAT31(local_4._1_3_,1);
    uVar3 = FUN_0054fd90(local_28,(int)local_44,iVar2,param_3);
    if (uVar3 != 0) {
      FUN_0054fae0(this,(int)local_44);
    }
    local_4 = local_4 & 0xffffff00;
    FUN_0054f880(local_44);
    local_4 = 0xffffffff;
    FUN_0054fa90(local_28);
    iVar1 = iVar1 + 1;
    iVar2 = iVar2 + 0xc;
    this = (void *)((int)this + 0x1c);
  } while (uVar3 != 0);
  ExceptionList = local_c;
  return 0;
}

