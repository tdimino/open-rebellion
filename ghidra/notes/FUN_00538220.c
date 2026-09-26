
int __thiscall
FUN_00538220(void *this,int *param_1,int *param_2,int *param_3,undefined4 *param_4,void *param_5)

{
  ushort uVar1;
  uint *puVar2;
  int iVar3;
  int iVar4;
  void *this_00;
  void *pvVar5;
  undefined3 extraout_var;
  uint uVar6;
  int unaff_EBX;
  bool bVar7;
  int local_28;
  int iStack_24;
  uint uStack_20;
  undefined1 local_1c [4];
  undefined4 uStack_18;
  undefined4 local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00645e68;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = 0;
  bVar7 = true;
  *param_2 = 0;
  *param_3 = 0;
  *param_4 = 0;
  local_14 = 1;
  local_10 = (void *)0xffffffff;
  local_4 = 0;
  FUN_00520580(param_5,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar2 = FUN_00451110(this,local_1c);
  local_14 = 0x10;
  local_10 = (void *)0x40;
  local_4 = 1;
  if ((*puVar2 >> 0x18 < 0x10) || (local_28 = 1, 0x3f < *puVar2 >> 0x18)) {
    local_28 = 0;
  }
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  if (local_28 != 0) {
    puVar2 = FUN_00451110(this,local_1c);
    local_4 = 2;
    iVar3 = FUN_0051cab0(puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar7 = iVar3 != 0;
    if (iVar3 != 0) {
      iVar4 = FUN_0053b870(iVar3);
      *param_1 = iVar4 * *(int *)((int)this + 0x48);
      iVar3 = FUN_0053b860(iVar3);
      *param_2 = iVar3 * *(int *)((int)this + 0x48);
    }
  }
  (**(code **)(*(int *)this + 0x1c))(param_5);
  iVar3 = FUN_00506e60();
  if ((iVar3 == 0) || (iVar4 = 1, !bVar7)) {
    iVar4 = 0;
  }
  if ((*(int *)((int)param_5 + 4) == -1) && (iVar3 != 0)) {
    this_00 = (void *)FUN_0041c210(*(int *)((int)this + 0x40));
    while ((this_00 != (void *)0x0 && (*(int *)((int)param_5 + 4) == -1))) {
      FUN_004fa150(this_00,&uStack_20);
      puStack_8 = (undefined1 *)0x3;
      pvVar5 = (void *)FUN_00505750(&uStack_20);
      if (pvVar5 != (void *)0x0) {
        uVar1 = FUN_005f50e0((int)this_00 + 0x50);
        local_28 = 0;
        iStack_24 = 0;
        if (*(int *)((int)param_5 + 4) == -1) {
          puVar2 = FUN_00451110(this,local_1c);
          puStack_8._0_1_ = 4;
          bVar7 = FUN_00555f00(pvVar5,puVar2,(uint)uVar1,param_5,&local_28);
          if ((CONCAT31(extraout_var,bVar7) == 0) || (iVar3 = 1, iVar4 == 0)) {
            iVar3 = 0;
          }
          puStack_8._0_1_ = 3;
          FUN_00619730();
          iVar4 = iVar3;
          if (*(int *)((int)param_5 + 4) == -1) {
            puVar2 = FUN_00451110(this,&uStack_18);
            puStack_8._0_1_ = 5;
            uVar6 = FUN_00555f90(pvVar5,puVar2,param_5,&iStack_24);
            if ((uVar6 == 0) || (iVar4 = 1, iVar3 == 0)) {
              iVar4 = 0;
            }
            puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,3);
            FUN_00619730();
            if (*(int *)((int)param_5 + 4) == -1) {
              param_4 = (undefined4 *)FUN_0053e130((int)param_4,local_28);
              unaff_EBX = FUN_0053e130(unaff_EBX,iStack_24);
            }
          }
        }
      }
      puStack_8 = (undefined1 *)0xffffffff;
      FUN_00619730();
      this_00 = *(void **)((int)this_00 + 0x10);
    }
  }
  if (iVar4 == 0) {
    uStack_18 = 1;
    local_14 = 1;
    puStack_8 = (undefined1 *)0x6;
    FUN_00520580(param_5,&uStack_18);
    puStack_8 = (undefined1 *)0xffffffff;
    FUN_00619730();
  }
  if (*(int *)((int)param_5 + 4) == -1) {
    *param_2 = (int)param_4;
    *param_3 = unaff_EBX;
  }
  ExceptionList = local_10;
  return iVar4;
}

