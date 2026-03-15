
void __thiscall FUN_004611d0(void *this,uint param_1)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  bool bVar5;
  int *this_00;
  uint *puVar6;
  void *pvVar7;
  char *pcVar8;
  int iVar9;
  void *pvVar10;
  uint local_3c;
  uint local_38;
  uint local_34;
  uint local_30;
  uint local_2c;
  void *local_28;
  uint local_24;
  undefined4 local_20;
  RECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006311ea;
  local_c = ExceptionList;
  bVar5 = false;
  bVar4 = false;
  bVar3 = false;
  bVar2 = false;
  bVar1 = false;
  iVar9 = *(int *)(*(int *)((int)this + 0x150) + 0x9c);
  if (*(uint *)((int)this + 0x158) != param_1) {
    ExceptionList = &local_c;
    FUN_005f5b20(*(void **)((int)this + 0x15c));
    this_00 = FUN_004f31b0(iVar9);
    while (this_00 != (int *)0x0) {
      pvVar10 = (void *)0x0;
      switch(param_1) {
      case 1:
        pvVar10 = (void *)FUN_00618b70(0x74);
        local_4 = 0;
        local_28 = pvVar10;
        if (pvVar10 == (void *)0x0) {
          pvVar10 = (void *)0x0;
        }
        else {
          puVar6 = FUN_004025b0(this_00,&local_3c);
          bVar1 = true;
          local_4 = CONCAT31(local_4._1_3_,1);
          pvVar10 = FUN_004421d0(pvVar10,puVar6);
        }
        local_4 = 0;
        if (bVar1) {
          bVar1 = false;
LAB_00461453:
          FUN_00619730();
        }
LAB_00461458:
        local_4 = 0xffffffff;
        break;
      case 2:
        if (((byte)this_00[9] & 0xc0) == 0x40) {
          pvVar10 = (void *)FUN_00618b70(0x74);
          local_4 = 2;
          local_28 = pvVar10;
          if (pvVar10 == (void *)0x0) {
            pvVar10 = (void *)0x0;
          }
          else {
            puVar6 = FUN_004025b0(this_00,&local_38);
            bVar2 = true;
            local_4 = CONCAT31(local_4._1_3_,3);
            pvVar10 = FUN_004421d0(pvVar10,puVar6);
          }
          local_4 = 2;
          if (bVar2) {
            bVar2 = false;
            goto LAB_00461453;
          }
          goto LAB_00461458;
        }
        break;
      case 3:
        if (((byte)this_00[9] & 0xc0) == 0x80) {
          pvVar10 = (void *)FUN_00618b70(0x74);
          local_4 = 4;
          local_28 = pvVar10;
          if (pvVar10 == (void *)0x0) {
            pvVar10 = (void *)0x0;
          }
          else {
            puVar6 = FUN_004025b0(this_00,&local_34);
            bVar3 = true;
            local_4 = CONCAT31(local_4._1_3_,5);
            pvVar10 = FUN_004421d0(pvVar10,puVar6);
          }
          local_4 = 4;
          if (bVar3) {
            bVar3 = false;
            goto LAB_00461453;
          }
          goto LAB_00461458;
        }
        break;
      case 4:
        if ((((byte)this_00[9] & 0xc0) == 0xc0) && ((*(byte *)(this_00 + 0x22) & 2) != 0)) {
          pvVar10 = (void *)FUN_00618b70(0x74);
          local_4 = 6;
          local_28 = pvVar10;
          if (pvVar10 == (void *)0x0) {
            pvVar10 = (void *)0x0;
          }
          else {
            puVar6 = FUN_004025b0(this_00,&local_30);
            bVar4 = true;
            local_4 = CONCAT31(local_4._1_3_,7);
            pvVar10 = FUN_004421d0(pvVar10,puVar6);
          }
          local_4 = 6;
          if (bVar4) {
            bVar4 = false;
            goto LAB_00461453;
          }
          goto LAB_00461458;
        }
        break;
      case 5:
        if ((*(byte *)(this_00 + 0x22) & 2) == 0) {
          pvVar10 = (void *)FUN_00618b70(0x74);
          local_4 = 8;
          local_28 = pvVar10;
          if (pvVar10 == (void *)0x0) {
            pvVar10 = (void *)0x0;
          }
          else {
            puVar6 = FUN_004025b0(this_00,&local_2c);
            bVar5 = true;
            local_4 = CONCAT31(local_4._1_3_,9);
            pvVar10 = FUN_004421d0(pvVar10,puVar6);
          }
          local_4 = 8;
          if (bVar5) {
            bVar5 = false;
            goto LAB_00461453;
          }
          goto LAB_00461458;
        }
      }
      if (pvVar10 != (void *)0x0) {
        pvVar7 = FUN_004f6270(this_00,&local_1c);
        local_4 = 10;
        pcVar8 = (char *)FUN_00583c40((int)pvVar7);
        FUN_005f35e0((void *)((int)pvVar10 + 0x14),pcVar8);
        local_4 = 0xffffffff;
        FUN_005f2ff0(&local_1c.left);
        FUN_005f59f0(*(void **)((int)this + 0x15c),(int)pvVar10);
      }
      local_24 = 0x90;
      local_20 = 0x98;
      local_4 = 0xb;
      this_00 = (int *)FUN_004f6010(this_00,&local_24,1);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    *(uint *)((int)this + 0x158) = param_1;
    FUN_0060a280(*(void **)((int)this + 0x14c));
    iVar9 = FUN_00604500((void *)(*(int *)((int)this + 0x144) + 0x6c),param_1);
    FUN_00601aa0(*(void **)((int)this + 0x154),*(char **)(iVar9 + 0x84));
    iVar9 = *(int *)((int)this + 0x154);
    local_1c.left = *(LONG *)(iVar9 + 0x14);
    local_1c.top = *(LONG *)(iVar9 + 0x18);
    local_1c.right = *(LONG *)(iVar9 + 0x1c);
    local_1c.bottom = *(LONG *)(iVar9 + 0x20);
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_1c,0);
  }
  ExceptionList = local_c;
  return;
}

