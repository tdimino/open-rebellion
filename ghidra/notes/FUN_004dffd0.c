
undefined4 __fastcall FUN_004dffd0(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  undefined4 *puVar3;
  void *pvVar4;
  void *pvVar5;
  int *this_00;
  uint *puVar6;
  undefined4 uVar7;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d8e8;
  local_c = ExceptionList;
  uVar7 = 0;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 100) = 0;
  puVar3 = FUN_00419b80(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x58),0x1000,0,0,0,0,0,2
                       );
  pvVar5 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar5,(int)puVar3);
  puVar3 = FUN_004357b0(pvVar5,&local_14);
  this = (uint *)(param_1 + 0x50);
  local_4 = 0;
  FUN_004f26d0(this,puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  pvVar4 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x800800,0,1,3,0,0,6,(void *)0x1);
  FUN_00435790(pvVar5,(int)pvVar4);
  puVar3 = FUN_004357b0(pvVar5,&local_14);
  local_4 = 1;
  FUN_004f26d0(this,puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  local_14 = (undefined1 *)0x90;
  local_10 = 0x98;
  if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)(param_1 + 0x2c);
    local_14 = &stack0xffffffd8;
    FUN_004f26d0(&stack0xffffffd8,this);
    pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
    this_00 = FUN_004f3220(*(int *)(param_1 + 0x30),this);
    if ((this_00 != (int *)0x0) && (pvVar5 != (void *)0x0)) {
      pvVar4 = (void *)FUN_00509670(this_00,1);
      if (pvVar4 != (void *)0x0) {
        iVar1 = *(int *)((int)pvVar5 + 0x60);
        *(int *)(param_1 + 100) = iVar1;
        if (1 < iVar1) {
          *(int *)(param_1 + 100) = iVar1 + -1;
        }
        puVar6 = FUN_004025b0(pvVar4,(uint *)&local_14);
        local_4 = 2;
        FUN_004f26d0(this,puVar6);
        local_4 = 0xffffffff;
        FUN_00619730();
        uVar7 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return uVar7;
}

