
undefined4 __fastcall FUN_004e21d0(int param_1)

{
  int iVar1;
  uint uVar2;
  bool bVar3;
  bool bVar4;
  void *this;
  undefined4 *puVar5;
  void *pvVar6;
  int *this_00;
  void *pvVar7;
  uint *puVar8;
  undefined4 local_2c;
  undefined1 local_28 [4];
  undefined1 local_24 [4];
  undefined1 local_20 [4];
  uint local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063dda8;
  local_c = ExceptionList;
  local_2c = 0;
  pvVar7 = (void *)(param_1 + 0x60);
  bVar4 = false;
  ExceptionList = &local_c;
  this = (void *)thunk_FUN_005f5060((int)pvVar7);
joined_r0x004e220a:
  do {
    if ((this == (void *)0x0) || (bVar3 = false, bVar4)) {
      if (bVar4) {
        this_00 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x4c));
        if ((this_00 != (int *)0x0) &&
           (pvVar7 = (void *)FUN_00509670(this_00,1), pvVar7 != (void *)0x0)) {
          puVar8 = FUN_004025b0(pvVar7,&local_1c);
          local_4 = 4;
          FUN_004f26d0((uint *)(param_1 + 0x4c),puVar8);
          local_4 = 0xffffffff;
          FUN_00619730();
          local_2c = 1;
        }
      }
      else {
        FUN_004ece80((undefined4 *)(param_1 + 0x4c));
      }
      ExceptionList = local_c;
      return local_2c;
    }
    puVar5 = FUN_00403040(this,local_28);
    puVar8 = (uint *)(param_1 + 0x4c);
    local_4 = 0;
    FUN_004f26d0(puVar8,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    local_14 = 0x90;
    local_10 = 0x98;
    if ((0x8f < *puVar8 >> 0x18) && (*puVar8 >> 0x18 < 0x98)) {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      local_18 = &stack0xffffffb8;
      iVar1 = *(int *)(param_1 + 0x2c);
      FUN_004f26d0(&stack0xffffffb8,puVar8);
      pvVar6 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar6 == (void *)0x0) {
        pvVar6 = *(void **)((int)this + 0x10);
        puVar8 = FUN_00403040(this,local_20);
        local_4 = 2;
        FUN_004f58e0(pvVar7,puVar8);
      }
      else {
        if (((*(byte *)((int)pvVar6 + 0x30) & 1) != 0) &&
           (uVar2 = *(uint *)((int)pvVar6 + 0x28), (uVar2 & 0x1000000) != 0)) {
          if (((0 < *(int *)((int)pvVar6 + 0x60)) && ((uVar2 & 0x800) != 0)) && ((uVar2 & 3) == 0))
          {
            bVar4 = true;
          }
          this = *(void **)((int)this + 0x10);
          goto joined_r0x004e220a;
        }
        *(uint *)((int)pvVar6 + 0x28) = *(uint *)((int)pvVar6 + 0x28) & 0xfeffffff;
        FUN_00433e30((int)pvVar6);
        pvVar6 = *(void **)((int)this + 0x10);
        puVar8 = FUN_00403040(this,local_24);
        local_4 = 1;
        FUN_004f58e0(pvVar7,puVar8);
      }
    }
    else {
      pvVar6 = *(void **)((int)this + 0x10);
      puVar8 = FUN_00403040(this,&local_1c);
      local_4 = 3;
      FUN_004f58e0(pvVar7,puVar8);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    this = pvVar6;
  } while( true );
}

