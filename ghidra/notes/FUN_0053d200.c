
bool __thiscall FUN_0053d200(void *this,void *param_1,int *param_2)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  void *pvVar4;
  uint *puVar5;
  int *piVar6;
  int *piVar7;
  undefined3 extraout_var;
  bool bVar8;
  undefined1 auStack_1c [4];
  undefined1 auStack_18 [4];
  undefined4 local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar4 = param_1;
  puStack_8 = &LAB_00646650;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = (void *)0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar6 = param_2;
  (**(code **)(*param_2 + 4))();
  iVar2 = (**(code **)(*(int *)this + 0x1c))(pvVar4);
  iVar3 = FUN_00506e60();
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar8 = false;
  }
  else {
    bVar8 = true;
  }
  if (((bVar8) && (iVar3 != 0)) && (*(int *)((int)pvVar4 + 4) == -1)) {
    iVar2 = (**(code **)(*(int *)this + 0x3c))(piVar6);
    bVar8 = iVar2 != 0;
    for (pvVar4 = (void *)thunk_FUN_005f5060((int)piVar6); pvVar4 != (void *)0x0;
        pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
      puVar5 = FUN_00403040(pvVar4,&stack0xffffffdc);
      puStack_8 = (undefined1 *)0x1;
      piVar6 = (int *)FUN_00504dc0(puVar5);
      puStack_8 = (undefined1 *)0xffffffff;
      FUN_00619730();
      if ((piVar6 == (int *)0x0) || (bVar1 = !bVar8, bVar8 = true, bVar1)) {
        bVar8 = false;
      }
      param_1 = (void *)0x0;
      if (piVar6 != (int *)0x0) {
        piVar7 = (int *)(**(code **)(*(int *)this + 0x30))(auStack_18);
        puStack_8 = (undefined1 *)0x2;
        puVar5 = FUN_00403040(pvVar4,auStack_1c);
        puStack_8._0_1_ = 3;
        bVar1 = FUN_00555d30(piVar6,puVar5,piVar7,&param_1);
        if ((CONCAT31(extraout_var,bVar1) == 0) || (bVar1 = !bVar8, bVar8 = true, bVar1)) {
          bVar8 = false;
        }
        puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,2);
        FUN_00619730();
        puStack_8 = (undefined1 *)0xffffffff;
        FUN_00619730();
      }
      *(void **)((int)pvVar4 + 0x1c) = param_1;
    }
  }
  ExceptionList = local_10;
  return bVar8;
}

