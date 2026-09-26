
int FUN_00582440(int param_1,int param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  void *this;
  uint *puVar4;
  int iVar5;
  int iVar6;
  void *pvVar7;
  undefined3 extraout_var;
  bool bVar8;
  bool bVar9;
  int local_4c;
  undefined1 local_48 [4];
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c [2];
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064ef08;
  pvStack_c = ExceptionList;
  local_44 = 1;
  local_40 = 0xffffffff;
  bVar8 = false;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_3,&local_44);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_4c = 0;
  bVar2 = false;
  bVar9 = *(int *)(param_1 + 0xc) == 0x223;
  FUN_004f4340(local_3c);
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  iVar3 = FUN_0053f150(param_1,param_2,local_3c,param_3);
  if ((iVar3 != 0) && (*(int *)((int)param_3 + 4) == -1)) {
    this = (void *)thunk_FUN_005f5060((int)local_3c);
    while ((!bVar2 && (this != (void *)0x0))) {
      puVar4 = FUN_00403040(this,local_48);
      local_34 = 8;
      local_30 = 0x10;
      local_4._0_1_ = 2;
      if ((*puVar4 >> 0x18 < 8) || (0xf < *puVar4 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      local_4._0_1_ = 1;
      FUN_00619730();
      if (bVar1) {
        puVar4 = FUN_00403040(this,&local_44);
        local_4._0_1_ = 3;
        iVar5 = FUN_00506480(puVar4);
        local_4._0_1_ = 1;
        FUN_00619730();
        if ((iVar5 == 0) || ((*(byte *)(iVar5 + 0x50) & 1) == 0)) goto LAB_005825f3;
        iVar6 = FUN_004fd6b0(iVar5);
        bVar8 = iVar6 != 0;
        pvVar7 = FUN_005003d0(local_2c,iVar5,1);
        local_4._0_1_ = 4;
        bVar2 = FUN_005131b0((int)pvVar7);
        local_4c = CONCAT31(extraout_var,bVar2);
        local_4._0_1_ = 1;
        FUN_005004d0(local_2c);
        if ((!bVar8) || ((bVar9 && (local_4c == 0)))) {
          bVar2 = false;
          goto LAB_005825f3;
        }
        this = *(void **)((int)this + 0x10);
        bVar2 = true;
      }
      else {
LAB_005825f3:
        this = *(void **)((int)this + 0x10);
      }
    }
  }
  if (((iVar3 != 0) && (*(int *)((int)param_3 + 4) == -1)) && (!bVar8)) {
    local_40 = 6;
    local_44 = 8;
    local_4._0_1_ = 5;
    FUN_00520580(param_3,&local_44);
    local_4._0_1_ = 1;
    FUN_00619730();
  }
  if (((iVar3 != 0) && (*(int *)((int)param_3 + 4) == -1)) && ((bVar9 && (local_4c == 0)))) {
    local_44 = 8;
    local_40 = 8;
    local_4._0_1_ = 6;
    FUN_00520580(param_3,&local_44);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_3c);
  ExceptionList = pvStack_c;
  return iVar3;
}

