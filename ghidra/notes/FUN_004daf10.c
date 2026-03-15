
undefined4 __fastcall FUN_004daf10(int param_1)

{
  int iVar1;
  bool bVar2;
  short sVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint *puVar6;
  void *pvVar7;
  int *this;
  uint *puVar8;
  uint uVar9;
  undefined4 local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d153;
  local_c = ExceptionList;
  local_18 = 0;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 0x54) = 0;
  sVar3 = FUN_005f50e0(param_1 + 0x58);
  if (sVar3 == 0) {
    pvVar4 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x2000,0,0,0,0,0,4,&DAT_00000002);
    pvVar7 = (void *)(param_1 + 0x68);
    FUN_00435790(pvVar7,(int)pvVar4);
    pvVar4 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x2000,0,0,0,0,0,0x15,(void *)0x1);
    FUN_00435790(pvVar7,(int)pvVar4);
    pvVar4 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x2000,0,0,0,0,0,8,(void *)0x1);
    FUN_00435790(pvVar7,(int)pvVar4);
    puVar5 = (undefined4 *)FUN_004357b0(pvVar7,&local_14);
    puVar8 = (uint *)(param_1 + 0x40);
    local_4 = 0;
    FUN_004f26d0(puVar8,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar7);
    pvVar4 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar8,0x800,0,1,0x3800000,0,0,6,(void *)0x1);
    FUN_00435790(pvVar7,(int)pvVar4);
    pvVar4 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar8,0x800,0,1,0x3800000,0,0,0,(void *)0x1);
    FUN_00435790(pvVar7,(int)pvVar4);
    pvVar4 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar8,0x800,0,1,0x3800000,0,0,0x33,(void *)0x1
                         );
    FUN_00435790(pvVar7,(int)pvVar4);
    puVar5 = (undefined4 *)FUN_004357b0(pvVar7,&local_14);
    local_4 = 1;
    FUN_004f26d0(puVar8,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar7);
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((*puVar8 >> 0x18 < 0x90) || (0x97 < *puVar8 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar4 = FUN_00419a00(*(void **)(param_1 + 0x2c),0,0,4,0,0,0,0x14,(void *)0x1);
      FUN_00435790(pvVar7,(int)pvVar4);
      puVar5 = (undefined4 *)FUN_004357b0(pvVar7,&local_14);
      local_4 = 2;
      FUN_004f26d0(puVar8,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar7);
      pvVar4 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar8,0,0,1,0,0,0,6,(void *)0x1);
      FUN_00435790(pvVar7,(int)pvVar4);
      puVar5 = (undefined4 *)FUN_004357b0(pvVar7,&local_14);
      local_4 = 3;
      FUN_004f26d0(puVar8,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar7);
    }
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((*puVar8 >> 0x18 < 0x90) || (0x97 < *puVar8 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      puVar6 = (uint *)(*(int *)(param_1 + 0x2c) + 4);
      *puVar6 = *puVar6 | 0x40;
    }
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((*puVar8 >> 0x18 < 0x90) || (0x97 < *puVar8 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x2c);
      local_14 = &stack0xffffffd0;
      FUN_004f26d0(&stack0xffffffd0,puVar8);
      pvVar7 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar7 != (void *)0x0) {
        local_14 = (undefined1 *)FUN_00618b70(0x1c);
        local_4 = 4;
        if (local_14 == (undefined1 *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          pvVar4 = FUN_004ecc70(local_14,puVar8);
        }
        local_4 = 0xffffffff;
        if (pvVar4 != (void *)0x0) {
          FUN_004f57b0((void *)(param_1 + 0x58),pvVar4);
          uVar9 = *(uint *)((int)pvVar7 + 0x28) & 0xfc7fffff;
          *(uint *)((int)pvVar7 + 0x28) = uVar9;
          *(uint *)((int)pvVar7 + 0x28) = uVar9 | 0x800000;
          FUN_00433e30((int)pvVar7);
          if ((*(uint *)((int)pvVar7 + 0x28) & 0x800) != 0) {
            local_18 = 1;
            this = FUN_004f3220(*(int *)(param_1 + 0x30),puVar8);
            if (this != (int *)0x0) {
              pvVar4 = (void *)FUN_00509670(this,1);
              if (pvVar4 != (void *)0x0) {
                *(undefined4 *)(param_1 + 0x54) = *(undefined4 *)((int)pvVar7 + 0x60);
                puVar8 = FUN_004025b0(pvVar4,(uint *)&local_14);
                local_4 = 5;
                FUN_004f26d0((void *)(param_1 + 0x44),puVar8);
                local_4 = 0xffffffff;
                FUN_00619730();
              }
            }
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return local_18;
}

