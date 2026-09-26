
undefined4 __fastcall FUN_004d0b40(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  int *this_00;
  uint *puVar6;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063bdb8;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 0x4c) = 0;
  pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x1000,0,0,0,0,0,0x14,(void *)0x1);
  pvVar5 = (void *)(param_1 + 0x68);
  FUN_00435790(pvVar5,(int)pvVar3);
  puVar4 = FUN_004357b0(pvVar5,&local_18);
  this = (uint *)(param_1 + 0x60);
  local_4 = 0;
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x800800,0,1,3,0,0,6,(void *)0x1);
  FUN_00435790(pvVar5,(int)pvVar3);
  puVar4 = FUN_004357b0(pvVar5,&local_18);
  local_4 = 1;
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  local_18 = (undefined1 *)0x90;
  local_14 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)(param_1 + 0x2c);
    local_18 = &stack0xffffffd8;
    FUN_004f26d0(&stack0xffffffd8,this);
    pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
    this_00 = FUN_004f3220(*(int *)(param_1 + 0x30),this);
    if ((this_00 != (int *)0x0) && (pvVar5 != (void *)0x0)) {
      pvVar3 = (void *)FUN_00509670(this_00,1);
      if (pvVar3 != (void *)0x0) {
        iVar1 = *(int *)((int)pvVar5 + 0x60);
        *(int *)(param_1 + 0x4c) = iVar1;
        if (1 < iVar1) {
          *(int *)(param_1 + 0x4c) = iVar1 + -1;
        }
        puVar6 = FUN_004025b0(pvVar3,(uint *)&local_18);
        local_4 = 2;
        FUN_004f26d0(this,puVar6);
        local_4 = 0xffffffff;
        FUN_00619730();
        ExceptionList = local_c;
        return 1;
      }
    }
  }
  ExceptionList = local_c;
  return local_10;
}

