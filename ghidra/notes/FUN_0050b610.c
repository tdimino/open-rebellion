
bool __thiscall FUN_0050b610(void *this,void *param_1)

{
  char cVar1;
  bool bVar2;
  bool bVar3;
  undefined3 extraout_var;
  void *pvVar4;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  uint uVar5;
  int iVar6;
  uint local_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641b98;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00558840(&local_30,(undefined4 *)((int)this + 0x84));
  local_4 = 0;
  if ((*(byte *)((int)this + 0x88) & 1) == 0) {
    uVar5 = 3;
  }
  else {
    cVar1 = FUN_0055a080(*(int *)((int)this + 0x58));
    uVar5 = CONCAT31(extraout_var,cVar1);
  }
  FUN_00558910(&local_30,uVar5);
  pvVar4 = FUN_00504cc0(local_2c,this,1,1);
  local_4._0_1_ = 1;
  bVar2 = FUN_005131b0((int)pvVar4);
  local_4._0_1_ = 0;
  FUN_00504d40(local_2c);
  pvVar4 = FUN_00504cc0(local_2c,this,1,2);
  local_4._0_1_ = 2;
  bVar3 = FUN_005131b0((int)pvVar4);
  iVar6 = CONCAT31(extraout_var_01,bVar3);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00504d40(local_2c);
  uVar5 = local_30 >> 4 & 3;
  if (CONCAT31(extraout_var_00,bVar2) == 0) {
    if (iVar6 == 0) {
      uVar5 = 0;
      goto LAB_0050b6f2;
    }
  }
  else {
    if (iVar6 == 0) {
      uVar5 = 1;
      goto LAB_0050b6f2;
    }
    if (CONCAT31(extraout_var_00,bVar2) != 0) goto LAB_0050b6f2;
  }
  if (iVar6 != 0) {
    uVar5 = 2;
  }
LAB_0050b6f2:
  FUN_00558930(&local_30,uVar5);
  if ((local_30 & 0x30) == 0) {
    iVar6 = 2;
  }
  else {
    iVar6 = 4;
  }
  FUN_005588f0(&local_30,local_30 >> iVar6 & 3);
  iVar6 = FUN_0050a300(this,(int *)&local_30,param_1);
  local_4 = 0xffffffff;
  thunk_FUN_00619730();
  ExceptionList = pvStack_c;
  return iVar6 != 0;
}

