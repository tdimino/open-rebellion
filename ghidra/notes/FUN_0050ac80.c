
int __thiscall FUN_0050ac80(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar2;
  int iVar3;
  undefined4 local_4c [8];
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641a80;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ffef0(local_2c,this,1,1);
  iVar2 = 0;
  local_4 = 0;
  FUN_004ffef0(local_4c,this,1,2);
  local_4 = CONCAT31(local_4._1_3_,1);
  bVar1 = FUN_005131b0((int)local_2c);
  if ((CONCAT31(extraout_var,bVar1) == 0) &&
     (((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) != 0x40)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  bVar1 = FUN_005131b0((int)local_4c);
  if ((CONCAT31(extraout_var_00,bVar1) != 0) ||
     (((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) == 0x80)) {
    iVar2 = 1;
  }
  iVar3 = FUN_004f7720(this,iVar3,param_1);
  if (iVar3 != 0) {
    iVar3 = FUN_004f7790(this,iVar2,param_1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_004fff70(local_4c);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  ExceptionList = pvStack_c;
  return iVar3;
}

