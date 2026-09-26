
int __fastcall FUN_004d8db0(int param_1)

{
  uint *puVar1;
  uint *this;
  int iVar2;
  bool bVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int *this_00;
  void *pvVar6;
  uint *puVar7;
  int local_1c;
  undefined1 *local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063cc93;
  local_c = ExceptionList;
  puVar1 = (uint *)(param_1 + 0x58);
  local_1c = 0;
  ExceptionList = &local_c;
  puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),puVar1,0x1000,0,0,0,0,0,2);
  pvVar5 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar5,(int)puVar4);
  puVar4 = FUN_004357b0(pvVar5,&local_18);
  local_4 = 0;
  this = (uint *)(param_1 + 0x50);
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  puVar4 = FUN_0041a400(*(void **)(param_1 + 0x2c),this,puVar1,0x800800,0,1,3,0,0,
                        (undefined4 *)&DAT_00000002);
  FUN_00435790(pvVar5,(int)puVar4);
  puVar4 = FUN_004357b0(pvVar5,&local_18);
  local_4 = 1;
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  local_14 = (undefined1 *)0x90;
  local_10 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    iVar2 = *(int *)(param_1 + 0x2c);
    local_18 = &stack0xffffffd0;
    FUN_004f26d0(&stack0xffffffd0,this);
    pvVar5 = FUN_00403d30((void *)(iVar2 + 0x2c));
    this_00 = FUN_004f3220(*(int *)(param_1 + 0x30),this);
    if ((this_00 != (int *)0x0) && (pvVar5 != (void *)0x0)) {
      pvVar6 = (void *)FUN_00509670(this_00,1);
      if (pvVar6 != (void *)0x0) {
        iVar2 = *(int *)((int)pvVar5 + 0x60);
        *(int *)(param_1 + 100) = iVar2;
        if (1 < iVar2) {
          *(int *)(param_1 + 100) = iVar2 + -1;
        }
        puVar7 = FUN_004025b0(pvVar6,(uint *)&local_18);
        local_4 = 2;
        FUN_004f26d0(this,puVar7);
        local_4 = 0xffffffff;
        FUN_00619730();
        local_1c = 1;
      }
    }
    if (local_1c != 0) {
      pvVar5 = FUN_004f5940((void *)(param_1 + 0x6c),puVar1);
      if (pvVar5 == (void *)0x0) {
        iVar2 = *(int *)(param_1 + 0x2c);
        local_14 = &stack0xffffffd0;
        FUN_004f26d0(&stack0xffffffd0,puVar1);
        pvVar5 = FUN_00403d30((void *)(iVar2 + 0x2c));
        if (pvVar5 != (void *)0x0) {
          local_14 = (undefined1 *)FUN_00618b70(0x1c);
          local_4 = 3;
          if (local_14 == (void *)0x0) {
            pvVar6 = (void *)0x0;
          }
          else {
            pvVar6 = FUN_004ecc70(local_14,puVar1);
          }
          local_4 = 0xffffffff;
          if (pvVar6 != (void *)0x0) {
            FUN_004f57b0((void *)(param_1 + 0x6c),pvVar6);
            *(uint *)((int)pvVar5 + 0x28) = *(uint *)((int)pvVar5 + 0x28) | 0x2000000;
            FUN_00433e30((int)pvVar5);
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return local_1c;
}

