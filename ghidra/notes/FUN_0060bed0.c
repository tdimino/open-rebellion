
void * __thiscall
FUN_0060bed0(void *this,undefined4 param_1,int param_2,undefined4 param_3,undefined4 param_4,
            LONG param_5,LONG param_6,undefined4 param_7,int *param_8,int param_9,
            undefined2 param_10,undefined2 param_11,undefined2 param_12,undefined2 param_13,
            undefined4 param_14)

{
  HGDIOBJ pvVar1;
  HDC hDC;
  void *pvVar2;
  int *piVar3;
  undefined4 uVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  uint uVar8;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656aac;
  local_c = ExceptionList;
  uVar8 = 0;
  uVar7 = 0;
  uVar6 = 0;
  iVar5 = 0;
  uVar4 = 0;
  ExceptionList = &local_c;
  pvVar1 = GetStockObject(4);
  FUN_005ff910(this,param_1,s_CoolSelectionBoxClass_006ac918,0x52000000,param_3,param_4,param_5,
               param_6,param_2,(char *)0x0,pvVar1,uVar4,iVar5,uVar6,uVar7,uVar8);
  local_4 = 0;
  FUN_0060a790((void *)((int)this + 0xd0),param_14);
  local_4._0_1_ = 1;
  *(undefined ***)this = &PTR_FUN_0066e1c8;
  *(undefined4 *)((int)this + 0x94) = 0xffffff;
  *(undefined4 *)((int)this + 0x98) = 0x24;
  *(undefined1 *)((int)this + 0xac) = 0;
  *(undefined4 *)((int)this + 0x9c) = 0;
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(LONG *)((int)this + 0xa4) = param_6;
  *(undefined4 *)((int)this + 0xa8) = param_7;
  pvVar1 = GetStockObject(6);
  *(HGDIOBJ *)((int)this + 0xb0) = pvVar1;
  *(undefined2 *)((int)this + 0xbc) = param_12;
  *(undefined2 *)((int)this + 0xb8) = param_10;
  *(undefined1 **)((int)this + 0xb4) = &LAB_005fef30;
  *(undefined4 *)((int)this + 0xe0) = 0;
  *(undefined4 *)((int)this + 0xe8) = 0;
  *(undefined2 *)((int)this + 0xba) = param_11;
  *(undefined2 *)((int)this + 0xbe) = param_13;
  *(undefined4 *)((int)this + 0xf0) = 0;
  *(int *)((int)this + 0xec) = param_9;
  if (param_9 != 0) {
    hDC = GetDC(*(HWND *)(param_2 + 0x18));
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4._0_1_ = 2;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005fbea0(pvVar2,param_5,param_6,*(uint **)((int)this + 0xec),hDC);
    }
    *(void **)((int)this + 0xf0) = pvVar2;
    local_4._0_1_ = 1;
    ReleaseDC(*(HWND *)(param_2 + 0x18),hDC);
  }
  *(undefined4 *)((int)this + 0xf4) = 0;
  *(undefined4 *)((int)this + 0xc0) = 0;
  iVar5 = GetSystemMetrics(0x10);
  *(int *)((int)this + 200) = iVar5;
  *(undefined4 *)((int)this + 0xc4) = 0;
  iVar5 = GetSystemMetrics(0x11);
  *(int *)((int)this + 0xcc) = iVar5;
  *(undefined4 *)((int)this + 0xe4) = param_14;
  if (param_8 != (int *)0x0) {
    for (piVar3 = (int *)(**(code **)(*param_8 + 8))(); piVar3 != (int *)0x0;
        piVar3 = (int *)(**(code **)(*piVar3 + 0xc))()) {
      pvVar2 = (void *)FUN_00618b70(0x68);
      local_4._0_1_ = 3;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_0060bac0(pvVar2,(int)piVar3);
      }
      local_4._0_1_ = 1;
      FUN_005f59f0((void *)((int)this + 0xd0),(int)pvVar2);
    }
    uVar4 = (**(code **)(*(int *)((int)this + 0xd0) + 8))();
    *(undefined4 *)((int)this + 0xe0) = uVar4;
  }
  ExceptionList = local_c;
  return this;
}

