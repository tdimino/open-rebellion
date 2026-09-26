
void __thiscall FUN_00417cf0(void *this,void *param_1)

{
  void *pvVar1;
  void *unaff_EBX;
  int iVar2;
  int iVar3;
  int unaff_ESI;
  undefined4 local_18;
  undefined4 uStack_14;
  uint *puStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b0a8;
  puStack_c = ExceptionList;
  ExceptionList = &puStack_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  FUN_005f4d90(param_1,this);
  FUN_005f4d90(param_1,(int)this + 0x17c);
  FUN_005f4d90(param_1,(int)this + 0x180);
  FUN_005f4990((void *)((int)this + 4),param_1);
  FUN_005f4d90(param_1,(int)this + 0xc);
  FUN_005f4d90(param_1,(int)this + 0x10);
  FUN_005f4d90(param_1,(int)this + 8);
  FUN_004ece90((void *)((int)this + 0x14),param_1);
  FUN_004ece90((void *)((int)this + 0x18),param_1);
  FUN_004ece90((void *)((int)this + 0x1c),param_1);
  FUN_004ece90((void *)((int)this + 0x20),param_1);
  FUN_004ece90((void *)((int)this + 0x24),param_1);
  FUN_004ece90((void *)((int)this + 0x28),param_1);
  (**(code **)(*(int *)((int)this + 0x138) + 4))();
  FUN_0042dc80((void *)((int)this + 0x138),param_1);
  FUN_0042dd00((void *)((int)this + 0x138),*(int *)this);
  (**(code **)(*(int *)((int)this + 0x2c) + 0x10))();
  FUN_004308f0((void *)((int)this + 0x44),param_1,(void *)((int)this + 0x2c),this);
  (**(code **)(*(int *)((int)this + 0x58) + 0x10))();
  (**(code **)(*(int *)((int)this + 0x78) + 0x10))();
  FUN_004ece90((void *)((int)this + 0x130),param_1);
  (**(code **)(*(int *)((int)this + 0x8c) + 0x10))();
  FUN_004ece90((void *)((int)this + 0x134),param_1);
  FUN_0042f510((void *)((int)this + 0xa8),param_1);
  FUN_0042ee50((void *)((int)this + 0xec),param_1);
  FUN_0042f1f0((void *)((int)this + 0xd8),param_1);
  FUN_0042ec90((void *)((int)this + 0x11c),param_1);
  iVar2 = (int)this + 0x184;
  iVar3 = 100;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_005f4d90(param_1,(int)this + 0x314);
  FUN_005f4d90(param_1,(int)this + 0x318);
  FUN_005f4d90(param_1,(int)this + 0x31c);
  iVar2 = (int)this + 800;
  iVar3 = 0xd;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_005f4d90(param_1,(int)this + 0x354);
  FUN_005f4d90(param_1,(int)this + 0x358);
  iVar2 = (int)this + 0x35c;
  iVar3 = 2;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  iVar2 = (int)this + 0x364;
  iVar3 = 0xc;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_005f4d90(param_1,&stack0xffffffdc);
  if (unaff_ESI == 0) {
    *(undefined4 *)((int)this + 0x178) = 0;
  }
  else {
    FUN_004ece90(&stack0xffffffd8,param_1);
    FUN_004f26d0(&stack0xffffffc4,(undefined4 *)&stack0xffffffd8);
    pvVar1 = FUN_0041bd90((void *)((int)this + 0x44));
    *(void **)((int)this + 0x178) = pvVar1;
  }
  uStack_14 = 0xffffffff;
  *puStack_c = *puStack_c | 0x80000000;
  FUN_00619730();
  ExceptionList = unaff_EBX;
  return;
}

