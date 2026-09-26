
undefined4 __fastcall FUN_00418500(void *param_1)

{
  uint *this;
  bool bVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  int iVar5;
  void *pvVar6;
  void *pvVar7;
  int iVar8;
  undefined1 *this_00;
  undefined4 local_30;
  undefined1 local_2c [4];
  undefined1 local_28 [4];
  undefined1 auStack_24 [4];
  undefined1 auStack_20 [4];
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b120;
  local_c = ExceptionList;
  this_00 = (undefined1 *)0x0;
  iVar8 = 0;
  local_30 = 0;
  switch(*(undefined4 *)((int)param_1 + 0x180)) {
  case 1:
    ExceptionList = &local_c;
    if (*(int *)((int)param_1 + 0x178) == 0) {
      ExceptionList = &local_c;
      iVar5 = thunk_FUN_005f5060((int)param_1 + 0x44);
      *(int *)((int)param_1 + 0x178) = iVar5;
    }
    if (*(int *)((int)param_1 + 0x178) != 0) {
      do {
        if (iVar8 != 0) break;
        iVar8 = FUN_004321e0(*(void **)((int)param_1 + 0x178));
        if (iVar8 != 0) {
          *(uint *)((int)param_1 + 4) = *(uint *)((int)param_1 + 4) | 0xa0000000;
        }
        iVar5 = *(int *)(*(int *)((int)param_1 + 0x178) + 0x10);
        *(int *)((int)param_1 + 0x178) = iVar5;
      } while (iVar5 != 0);
      if (*(int *)((int)param_1 + 0x178) != 0) {
        ExceptionList = local_c;
        return 0;
      }
    }
    *(undefined4 *)((int)param_1 + 0x180) = 5;
    break;
  case 2:
    puVar3 = (undefined4 *)((int)param_1 + 0x130);
    uVar2 = *(uint *)((int)param_1 + 0x130) >> 0x18;
    local_14 = 8;
    local_10 = 0x10;
    if ((uVar2 < 8) || (bVar1 = true, 0xf < uVar2)) {
      bVar1 = false;
    }
    ExceptionList = &local_c;
    FUN_00619730();
    if (bVar1) {
      FUN_004f26d0(&stack0xffffffb8,puVar3);
      this_00 = FUN_00419e40((void *)((int)param_1 + 0x78));
    }
    if ((this_00 != (undefined1 *)0x0) ||
       (this_00 = (undefined1 *)thunk_FUN_005f5060((int)param_1 + 0x78),
       this_00 != (undefined1 *)0x0)) {
      while (iVar8 == 0) {
        iVar8 = FUN_00433620(this_00);
        if (iVar8 != 0) {
          *(uint *)((int)param_1 + 4) = *(uint *)((int)param_1 + 4) | 0xc0000000;
        }
        this_00 = *(undefined1 **)(this_00 + 0x10);
        if (this_00 == (undefined1 *)0x0) {
          FUN_004ece80(puVar3);
          break;
        }
        puVar4 = FUN_00403040(this_00,local_28);
        local_4 = 1;
        FUN_004f26d0(puVar3,puVar4);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      if (this_00 != (undefined1 *)0x0) {
        ExceptionList = local_c;
        return 0;
      }
    }
    *(undefined4 *)((int)param_1 + 0x180) = 1;
    break;
  default:
    *(undefined4 *)((int)param_1 + 0x180) = 4;
    break;
  case 4:
    this = (uint *)((int)param_1 + 0x134);
    uVar2 = *(uint *)((int)param_1 + 0x134) >> 0x18;
    local_1c = 0x30;
    local_18 = 0x40;
    if ((uVar2 < 0x30) || (bVar1 = true, 0x3f < uVar2)) {
      bVar1 = false;
    }
    ExceptionList = &local_c;
    FUN_00619730();
    if (bVar1) {
      this_00 = FUN_004f5940((void *)((int)param_1 + 0x8c),this);
    }
    if ((this_00 != (undefined1 *)0x0) ||
       (this_00 = (undefined1 *)thunk_FUN_005f5060((int)param_1 + 0x8c),
       this_00 != (undefined1 *)0x0)) {
      while (iVar8 == 0) {
        iVar8 = FUN_00402230(this_00);
        if (iVar8 != 0) {
          *(uint *)((int)param_1 + 4) = *(uint *)((int)param_1 + 4) | 0x90000000;
        }
        this_00 = *(undefined1 **)(this_00 + 0x10);
        if (this_00 == (undefined1 *)0x0) {
          FUN_004ece80(this);
          break;
        }
        puVar3 = FUN_00403040(this_00,local_2c);
        local_4 = 0;
        FUN_004f26d0(this,puVar3);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      if (this_00 != (undefined1 *)0x0) {
        ExceptionList = local_c;
        return 0;
      }
    }
    *(undefined4 *)((int)param_1 + 0x180) = 2;
    break;
  case 5:
    if ((*(uint *)((int)param_1 + 4) & 0x80000000) != 0) {
      ExceptionList = &local_c;
      (**(code **)(*(int *)((int)param_1 + 0xc0) + 4))();
      (**(code **)(*(int *)((int)param_1 + 0x104) + 4))();
      if ((*(uint *)((int)param_1 + 4) & 0x20000000) != 0) {
        puVar3 = (undefined4 *)((int)param_1 + 0x1ac);
        for (iVar8 = 0x28; iVar8 != 0; iVar8 = iVar8 + -1) {
          *puVar3 = 0;
          puVar3 = puVar3 + 1;
        }
        *(uint *)((int)param_1 + 4) = *(uint *)((int)param_1 + 4) & 0xfffffa6f;
        iVar8 = thunk_FUN_005f5060((int)param_1 + 0x44);
        *(int *)((int)param_1 + 0x178) = iVar8;
        while (iVar8 != 0) {
          iVar8 = *(int *)((int)param_1 + 0x178);
          FUN_0041b7e0(param_1,iVar8 + 0x70,(uint *)(iVar8 + 0x24),(uint *)(iVar8 + 0x2c),
                       (uint *)(iVar8 + 0x28));
          iVar8 = *(int *)(*(int *)((int)param_1 + 0x178) + 0x10);
          *(int *)((int)param_1 + 0x178) = iVar8;
        }
        if (0 < *(int *)((int)param_1 + 0x214)) {
          *(int *)((int)param_1 + 0x214) = *(int *)((int)param_1 + 0x214) / 5 + 1;
        }
        if (0 < *(int *)((int)param_1 + 0x218)) {
          *(int *)((int)param_1 + 0x218) = *(int *)((int)param_1 + 0x218) / 6 + 1;
        }
      }
      if ((*(uint *)((int)param_1 + 4) & 0x40000000) != 0) {
        puVar3 = (undefined4 *)((int)param_1 + 0x24c);
        for (iVar8 = 0x1e; iVar8 != 0; iVar8 = iVar8 + -1) {
          *puVar3 = 0;
          puVar3 = puVar3 + 1;
        }
        *(uint *)((int)param_1 + 4) = *(uint *)((int)param_1 + 4) & 0xfffffff3;
        for (pvVar6 = (void *)thunk_FUN_005f5060((int)param_1 + 0x78); pvVar6 != (void *)0x0;
            pvVar6 = *(void **)((int)pvVar6 + 0x10)) {
          pvVar7 = FUN_00403040(pvVar6,auStack_24);
          local_4 = 2;
          FUN_0041ba80(param_1,pvVar7,(int)pvVar6 + 0x50,(uint *)((int)pvVar6 + 0x38));
          local_4 = 0xffffffff;
          FUN_00619730();
        }
      }
      if ((*(uint *)((int)param_1 + 4) & 0x10000000) != 0) {
        puVar3 = (undefined4 *)((int)param_1 + 0x2c4);
        for (iVar8 = 0x14; iVar8 != 0; iVar8 = iVar8 + -1) {
          *puVar3 = 0;
          puVar3 = puVar3 + 1;
        }
        *(uint *)((int)param_1 + 4) = *(uint *)((int)param_1 + 4) & 0xfffffdff;
        for (pvVar6 = (void *)thunk_FUN_005f5060((int)param_1 + 0x8c); pvVar6 != (void *)0x0;
            pvVar6 = *(void **)((int)pvVar6 + 0x10)) {
          pvVar7 = FUN_00403040(pvVar6,auStack_20);
          local_4 = 3;
          FUN_0041bc10(param_1,pvVar7,(int)pvVar6 + 0x40,(uint *)((int)pvVar6 + 0x30),
                       (uint *)((int)pvVar6 + 0x34));
          local_4 = 0xffffffff;
          FUN_00619730();
        }
      }
      *(uint *)((int)param_1 + 4) = *(uint *)((int)param_1 + 4) & 0xfffffff;
    }
    *(undefined4 *)((int)param_1 + 0x180) = 4;
    local_30 = 1;
  }
  ExceptionList = local_c;
  return local_30;
}

