
undefined4 __fastcall FUN_004daca0(int param_1)

{
  int iVar1;
  bool bVar2;
  short sVar3;
  void *this;
  undefined4 *puVar4;
  void *pvVar5;
  uint uVar6;
  int *this_00;
  void *pvVar7;
  uint *puVar8;
  undefined4 local_2c;
  undefined1 local_28 [4];
  undefined1 local_24 [4];
  undefined1 local_20 [4];
  undefined1 local_1c [4];
  uint local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d110;
  local_c = ExceptionList;
  pvVar7 = (void *)(param_1 + 0x58);
  local_2c = 0;
  bVar2 = false;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 0x54) = 0;
  this = (void *)thunk_FUN_005f5060((int)pvVar7);
joined_r0x004dacd9:
  do {
    if ((this == (void *)0x0) || (bVar2)) {
      bVar2 = false;
      uVar6 = *(uint *)(param_1 + 0x44) >> 0x18;
      local_14 = (undefined1 *)0x90;
      local_10 = 0x98;
      if ((0x8f < uVar6) && (uVar6 < 0x98)) {
        bVar2 = true;
      }
      FUN_00619730();
      if (((bVar2) &&
          (this_00 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x44)),
          this_00 != (int *)0x0)) &&
         (pvVar7 = (void *)FUN_00509670(this_00,1), pvVar7 != (void *)0x0)) {
        puVar8 = FUN_004025b0(pvVar7,&local_18);
        local_4 = 5;
        FUN_004f26d0((uint *)(param_1 + 0x44),puVar8);
        local_4 = 0xffffffff;
        FUN_00619730();
        local_2c = 1;
      }
      ExceptionList = local_c;
      return local_2c;
    }
    puVar4 = FUN_00403040(this,local_28);
    local_14 = &stack0xffffffbc;
    local_4 = 0;
    iVar1 = *(int *)(param_1 + 0x2c);
    FUN_004f26d0(&stack0xffffffbc,puVar4);
    pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar2 = false;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = *(void **)((int)this + 0x10);
      puVar8 = FUN_00403040(this,&local_18);
      local_4 = 4;
      FUN_004f58e0(pvVar7,puVar8);
    }
    else {
      if (((*(byte *)((int)pvVar5 + 0x30) & 1) != 0) &&
         (uVar6 = *(uint *)((int)pvVar5 + 0x28), (uVar6 & 0x800000) != 0)) {
        if ((uVar6 & 0x800) == 0) {
          sVar3 = FUN_005f50e0((int)pvVar7);
          if ((sVar3 == 1) && (*(int *)((int)pvVar5 + 0x60) == 0)) {
            *(uint *)((int)pvVar5 + 0x28) = *(uint *)((int)pvVar5 + 0x28) & 0xff7fffff;
            FUN_00433e30((int)pvVar5);
            pvVar5 = *(void **)((int)this + 0x10);
            puVar8 = FUN_00403040(this,local_20);
            local_4 = 2;
            FUN_004f58e0(pvVar7,puVar8);
            goto LAB_004dae58;
          }
        }
        else if ((uVar6 & 3) == 0) {
          *(undefined4 *)(param_1 + 0x54) = *(undefined4 *)((int)pvVar5 + 0x60);
          puVar4 = FUN_00403040(this,local_24);
          local_4 = 1;
          FUN_004f26d0((void *)(param_1 + 0x44),puVar4);
          local_4 = 0xffffffff;
          FUN_00619730();
          bVar2 = true;
          this = *(void **)((int)this + 0x10);
          goto joined_r0x004dacd9;
        }
        this = *(void **)((int)this + 0x10);
        goto joined_r0x004dacd9;
      }
      *(uint *)((int)pvVar5 + 0x28) = *(uint *)((int)pvVar5 + 0x28) & 0xff7fffff;
      FUN_00433e30((int)pvVar5);
      pvVar5 = *(void **)((int)this + 0x10);
      puVar8 = FUN_00403040(this,local_1c);
      local_4 = 3;
      FUN_004f58e0(pvVar7,puVar8);
    }
LAB_004dae58:
    local_4 = 0xffffffff;
    FUN_00619730();
    this = pvVar5;
  } while( true );
}

