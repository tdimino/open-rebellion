
void __thiscall FUN_00417fa0(void *this,void *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  void *unaff_EDI;
  void *pvVar4;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b0d0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_005f4db0(param_1,this);
  FUN_005f4db0(param_1,(int)this + 0x17c);
  FUN_005f4db0(param_1,(int)this + 0x180);
  FUN_005f49a0((void *)((int)this + 4),param_1);
  FUN_005f4db0(param_1,(int)this + 0xc);
  FUN_005f4db0(param_1,(int)this + 0x10);
  FUN_005f4db0(param_1,(int)this + 8);
  FUN_004ecea0((void *)((int)this + 0x14),param_1);
  FUN_004ecea0((void *)((int)this + 0x18),param_1);
  FUN_004ecea0((void *)((int)this + 0x1c),param_1);
  FUN_004ecea0((void *)((int)this + 0x20),param_1);
  FUN_004ecea0((void *)((int)this + 0x24),param_1);
  FUN_004ecea0((void *)((int)this + 0x28),param_1);
  FUN_0042dcc0((void *)((int)this + 0x138),param_1);
  pvVar4 = param_1;
  (**(code **)(*(int *)((int)this + 0x2c) + 0x18))();
  (**(code **)(*(int *)((int)this + 0x44) + 0x18))(param_1);
  (**(code **)(*(int *)((int)this + 0x58) + 0x18))(param_1);
  (**(code **)(*(int *)((int)this + 0x78) + 0x18))(param_1);
  FUN_004ecea0((void *)((int)this + 0x130),param_1);
  (**(code **)(*(int *)((int)this + 0x8c) + 0x18))(param_1);
  FUN_004ecea0((void *)((int)this + 0x134),param_1);
  FUN_0042f5b0((void *)((int)this + 0xa8),param_1);
  FUN_0042eee0((void *)((int)this + 0xec),param_1);
  FUN_0042f2b0((void *)((int)this + 0xd8),param_1);
  FUN_0042f2b0((void *)((int)this + 0x11c),param_1);
  iVar3 = (int)this + 0x184;
  iVar2 = 100;
  do {
    FUN_005f4db0(param_1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  FUN_005f4db0(param_1,(int)this + 0x314);
  FUN_005f4db0(param_1,(int)this + 0x318);
  FUN_005f4db0(param_1,(int)this + 0x31c);
  iVar3 = (int)this + 800;
  iVar2 = 0xd;
  do {
    FUN_005f4db0(param_1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  FUN_005f4db0(param_1,(int)this + 0x354);
  FUN_005f4db0(param_1,(int)this + 0x358);
  iVar3 = (int)this + 0x35c;
  iVar2 = 2;
  do {
    FUN_005f4db0(param_1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  iVar3 = (int)this + 0x364;
  iVar2 = 0xc;
  do {
    FUN_005f4db0(param_1,iVar3);
    iVar3 = iVar3 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  if (*(void **)((int)this + 0x178) == (void *)0x0) {
    local_10 = (void *)0x0;
    FUN_005f4db0(param_1,&local_10);
  }
  else {
    puVar1 = FUN_00403040(*(void **)((int)this + 0x178),&local_10);
    FUN_004f26d0(&stack0xffffffdc,puVar1);
    FUN_00619730();
    local_10 = (void *)((uint)pvVar4 >> 0x18);
    FUN_005f4db0(param_1,&local_10);
    FUN_004ecea0(&stack0xffffffdc,param_1);
  }
  FUN_00619730();
  ExceptionList = unaff_EDI;
  return;
}

