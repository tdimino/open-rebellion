
/* WARNING: Type propagation algorithm not settling */

bool __thiscall FUN_00547480(void *this,int param_1,uint *param_2,int param_3,uint param_4)

{
  bool bVar1;
  uint *puVar2;
  int iVar3;
  undefined4 *puVar4;
  int iVar5;
  uint local_2c;
  int local_28 [4];
  undefined4 local_18;
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006478a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_2c);
  puVar2 = param_2;
  bVar1 = false;
  local_4 = 0;
  FUN_004f26d0(local_28,param_2);
  local_4._0_1_ = 1;
  FUN_005834d0(local_28 + 3);
  iVar5 = param_3;
  local_28[1] = 1;
  local_18 = (uint)local_18._2_2_ << 0x10;
  local_28[2] = -1;
  local_4._0_1_ = 3;
  iVar3 = FUN_0054bf00(param_3,&local_2c,local_28 + 1);
  if (local_28[2] == -1) {
    puVar4 = FUN_004f4340(local_14);
    param_2 = (uint *)0x41000001;
    local_4._0_1_ = 5;
    iVar5 = FUN_00547ba0(this,0,0,param_1,puVar2,(uint *)&param_2,local_28,&local_2c,iVar5,puVar4,
                         local_28 + 3,0,1,param_4);
    if ((iVar5 == 0) || (iVar3 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4._0_1_ = 4;
    FUN_00619730();
    local_4._0_1_ = 3;
    FUN_004f4380(local_14);
  }
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00583500(local_28 + 3);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar1;
}

