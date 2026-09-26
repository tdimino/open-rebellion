
/* WARNING: Type propagation algorithm not settling */

bool __thiscall
FUN_005475f0(void *this,int param_1,void *param_2,uint *param_3,int param_4,int param_5,uint param_6
            )

{
  bool bVar1;
  uint *puVar2;
  int iVar3;
  undefined4 *puVar4;
  int iVar5;
  uint local_24;
  int local_20 [5];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006478d8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  puVar2 = param_3;
  bVar1 = false;
  local_4 = 0;
  FUN_004f26d0(local_20,param_3);
  iVar5 = param_4;
  local_20[1] = 1;
  local_20[2] = -1;
  local_4._0_1_ = 2;
  iVar3 = FUN_0054bf00(param_4,&local_24,local_20 + 1);
  if (local_20[2] == -1) {
    puVar4 = FUN_004f4340(local_20 + 3);
    param_3 = (uint *)0x42000002;
    local_4._0_1_ = 4;
    iVar5 = FUN_00547ba0(this,0,0,param_1,puVar2,(uint *)&param_3,local_20,&local_24,iVar5,puVar4,
                         param_2,0,param_5,param_6);
    if ((iVar5 == 0) || (iVar3 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4._0_1_ = 3;
    FUN_00619730();
    local_4._0_1_ = 2;
    FUN_004f4380(local_20 + 3);
  }
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar1;
}

