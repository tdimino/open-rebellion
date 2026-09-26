
uint __cdecl FUN_0056c510(uint param_1,void *param_2,undefined4 *param_3)

{
  bool bVar1;
  uint uVar2;
  void *this;
  undefined4 *puVar3;
  uint uVar4;
  int *piVar5;
  void *pvVar6;
  int iVar7;
  undefined1 *puVar8;
  bool bVar9;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this = param_2;
  puStack_8 = &LAB_0064bab8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar3 = param_3;
  uVar2 = param_1;
  *param_3 = 0;
  uVar4 = FUN_005868c0(param_1,this,param_3);
  if (*(int *)(uVar2 + 4) != 0) {
    if (*(int *)(uVar2 + 8) == 0) {
      if (uVar4 != 0) {
        if (*(int *)((int)this + 4) == -1) {
          uVar4 = FUN_0056c990(*(int *)(uVar2 + 0x18),uVar2,this,puVar3);
        }
        if (uVar4 != 0) {
          if (*(int *)((int)this + 4) == -1) {
            uVar4 = FUN_0056c990(*(int *)(uVar2 + 0x1c),uVar2,this,puVar3);
          }
          if ((uVar4 != 0) && (*(int *)((int)this + 4) == -1)) {
            uVar4 = FUN_0056c990(*(int *)(uVar2 + 0x20),uVar2,this,puVar3);
          }
        }
      }
    }
    else if (uVar4 != 0) {
      if (*(int *)((int)this + 4) == -1) {
        uVar4 = FUN_0056c820(*(int *)(uVar2 + 0x24),uVar2,this,puVar3);
      }
      if (uVar4 != 0) {
        if (*(int *)((int)this + 4) == -1) {
          uVar4 = FUN_0056c820(*(int *)(uVar2 + 0x28),uVar2,this,puVar3);
        }
        if ((uVar4 != 0) && (*(int *)((int)this + 4) == -1)) {
          uVar4 = FUN_0056c820(*(int *)(uVar2 + 0x2c),uVar2,this,puVar3);
        }
      }
    }
  }
  if ((*(int *)(uVar2 + 0xc) != 0) && (uVar4 != 0)) {
    if (*(int *)((int)this + 4) == -1) {
      param_2 = (void *)0x0;
      piVar5 = (int *)FUN_00402e40(*(void **)(uVar2 + 0x14),(uint *)&param_3);
      local_4 = 1;
      uVar4 = FUN_0056c460(piVar5,&param_2);
      local_4 = 0xffffffff;
      FUN_00619730();
      pvVar6 = param_2;
      if (uVar4 != 0) {
        pvVar6 = (void *)FUN_00509670(*(void **)(uVar2 + 0x38),(int)param_2);
      }
      iVar7 = 0;
      if (pvVar6 != (void *)0x0) {
        iVar7 = FUN_00528100(pvVar6,0);
      }
      if (iVar7 == 0) {
        local_14 = 0x40;
        local_10 = 0x2c;
        local_4 = 2;
        FUN_00520580(this,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *puVar3 = 0xc;
      }
    }
    if ((uVar4 != 0) && (*(int *)((int)this + 4) == -1)) {
      param_2 = (void *)0x0;
      piVar5 = (int *)FUN_00402e40(*(void **)(uVar2 + 0x14),&param_1);
      local_4 = 3;
      iVar7 = FUN_0056c460(piVar5,&param_2);
      local_4 = 0xffffffff;
      FUN_00619730();
      if (iVar7 == 0) {
        bVar1 = false;
        puVar8 = param_2;
      }
      else if (*(int *)(uVar2 + 0x10) == 1) {
        bVar1 = true;
        puVar8 = (undefined1 *)0x1;
      }
      else if (*(int *)(uVar2 + 0x10) == 2) {
        bVar1 = true;
        puVar8 = &DAT_00000002;
      }
      else {
        bVar1 = false;
        puVar8 = (undefined1 *)0x0;
      }
      pvVar6 = (void *)0x0;
      bVar9 = false;
      if (bVar1) {
        pvVar6 = (void *)FUN_00506f30((int)puVar8);
        bVar9 = pvVar6 != (void *)0x0;
      }
      param_3 = (undefined4 *)0x0;
      uVar4 = 0;
      if (((bVar9) && (uVar4 = FUN_0052eb10(pvVar6,(int)param_2,&param_3), uVar4 != 0)) &&
         (param_3 != (undefined4 *)0x0)) {
        local_14 = 0x40;
        local_10 = 0x32;
        local_4 = 4;
        FUN_00520580(this,&local_14);
        local_4 = 0xffffffff;
        FUN_00619730();
        *puVar3 = 0xe;
      }
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

