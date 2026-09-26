
void __thiscall FUN_004a0400(void *this,void *param_1,void *param_2)

{
  bool bVar1;
  BOOL BVar2;
  void *pvVar3;
  void *this_00;
  undefined4 *puVar4;
  int *piVar5;
  undefined1 auStack_24 [4];
  void *pvStack_20;
  undefined4 local_1c [2];
  int *piStack_14;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  pvVar3 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063753a;
  local_c = ExceptionList;
  bVar1 = false;
  if (param_2 != (void *)0x0) {
    ExceptionList = &local_c;
    FUN_0060a790(local_1c,1);
    piVar5 = (int *)((int)param_2 + 0x24);
    local_4 = 0;
    param_2 = (void *)0x0;
    if ((*piVar5 == *(int *)(*(int *)((int)this + 0x1c0) + 0x24)) &&
       (BVar2 = IsWindowVisible(*(HWND *)(*(int *)((int)this + 0x1c0) + 0x18)), BVar2 != 0)) {
      FUN_00609410(*(void **)((int)this + 0x1c0),local_1c);
      param_2 = *(void **)((int)this + 0x1bc);
      piVar5 = piStack_14;
    }
    else {
      piVar5 = piStack_14;
      if (*(int *)((int)pvVar3 + 0x24) == *(int *)((int)*(void **)((int)this + 0x1b8) + 0x24)) {
        FUN_00609410(*(void **)((int)this + 0x1b8),local_1c);
        param_2 = *(void **)((int)this + 0x1b4);
        piVar5 = piStack_14;
      }
    }
    for (; piVar5 != (int *)0x0; piVar5 = (int *)(**(code **)(*piVar5 + 0xc))()) {
      pvVar3 = (void *)FUN_0060a860(param_2,piVar5[3]);
      if (pvVar3 != (void *)0x0) {
        this_00 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 1;
        pvStack_20 = this_00;
        if (this_00 == (void *)0x0) {
          pvVar3 = (void *)0x0;
        }
        else {
          puVar4 = FUN_0042d170(pvVar3,auStack_24);
          bVar1 = true;
          local_4 = CONCAT31(local_4._1_3_,2);
          pvVar3 = FUN_004f5b10(this_00,puVar4,0);
        }
        local_4 = 1;
        if (bVar1) {
          bVar1 = false;
          FUN_00619730();
        }
        local_4 = local_4 & 0xffffff00;
        FUN_004f57b0(param_1,pvVar3);
      }
    }
    local_4 = 0xffffffff;
    FUN_0060a810(local_1c);
  }
  ExceptionList = local_c;
  return;
}

