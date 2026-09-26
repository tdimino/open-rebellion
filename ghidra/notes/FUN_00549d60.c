
undefined4 __thiscall FUN_00549d60(void *this,int *param_1,void *param_2)

{
  int *this_00;
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  uint uVar3;
  undefined4 uVar4;
  undefined4 local_28 [5];
  undefined4 local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647d18;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_004f9190(this,param_1,param_2);
  bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  if ((*(uint *)((int)this + 0x24) & 0x30) == 0) {
    iVar2 = FUN_0054cb70((int)this);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
  }
  this_00 = (int *)((int)this + 0x5c);
  FUN_00540230(this_00,DAT_006bb560);
  FUN_00540270(this_00,DAT_006bb5bc);
  FUN_004fd400(local_28);
  local_4 = 0;
  FUN_004fd490(local_28,param_2);
  local_14 = FUN_004fd2f0();
  uVar3 = FUN_0053fa60(0x392,1,this,this_00,local_28);
  if ((uVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00540230((void *)((int)this + 100),DAT_006bb550);
  FUN_00540270((void *)((int)this + 100),DAT_006bb548);
  FUN_004fd490(local_28,param_2);
  local_14 = FUN_004fd2f0();
  iVar2 = FUN_00543fc0(this,1,local_28);
  if ((iVar2 == 0) || (!bVar1)) {
    uVar4 = 0;
  }
  else {
    uVar4 = 1;
  }
  local_4 = 0xffffffff;
  FUN_004fd4d0(local_28);
  ExceptionList = local_c;
  return uVar4;
}

