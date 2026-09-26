
void * __thiscall FUN_0045c6b0(void *this,void *param_1,int param_2,int param_3)

{
  int iVar1;
  int iVar2;
  void *this_00;
  POINT pt;
  int iVar3;
  int iVar4;
  bool bVar5;
  int *this_01;
  void *this_02;
  BOOL BVar6;
  undefined4 *puVar7;
  undefined3 extraout_var;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  iVar4 = param_3;
  iVar3 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630b2f;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this_01 = FUN_0045c660(this,param_2,param_3);
  FUN_004ece30(&param_2);
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  if (this_01 == (int *)0x0) goto LAB_0045c7f2;
  this_02 = (void *)FUN_0060a860((void *)((int)this + 0x174),0x10000);
  if (this_02 == (void *)0x0) {
    puVar7 = FUN_0042d170(this_01,&param_3);
    local_4._0_1_ = 5;
    FUN_004f26d0(&param_2,puVar7);
  }
  else {
    pt.y = iVar4;
    pt.x = iVar3;
    BVar6 = PtInRect((RECT *)((int)this_02 + 0x40),pt);
    if (BVar6 == 0) {
      puVar7 = FUN_0042d170(this_01,&param_3);
      local_4._0_1_ = 4;
      FUN_004f26d0(&param_2,puVar7);
    }
    else {
      iVar1 = *(int *)((int)this_02 + 0x44);
      iVar2 = *(int *)((int)this_02 + 0x40);
      this_00 = *(void **)((int)this_02 + 0x20);
      puVar7 = FUN_0042d170(this_01,&param_3);
      local_4._0_1_ = 2;
      FUN_004f26d0(&param_2,puVar7);
      local_4._0_1_ = 1;
      FUN_00619730();
      if (this_00 == (void *)0x0) goto LAB_0045c7f2;
      bVar5 = FUN_005fca00(this_00,iVar3 - iVar2,iVar4 - iVar1);
      if (CONCAT31(extraout_var,bVar5) == 0) goto LAB_0045c7f2;
      puVar7 = FUN_0042d170(this_02,&param_3);
      local_4._0_1_ = 3;
      FUN_004f26d0(&param_2,puVar7);
    }
  }
  local_4._0_1_ = 1;
  FUN_00619730();
LAB_0045c7f2:
  FUN_004f26d0(param_1,&param_2);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return param_1;
}

