
void __thiscall FUN_00476520(void *this,void *param_1)

{
  int *piVar1;
  int iVar2;
  int unaff_EBP;
  int iVar3;
  
  FUN_005f5610(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x1c);
  FUN_005f4d90(param_1,(int)this + 0x20);
  FUN_005f4d90(param_1,(int)this + 0x44);
  FUN_005f4990((void *)((int)this + 0x60),param_1);
  FUN_005f4990((void *)((int)this + 100),param_1);
  FUN_004ece90((void *)((int)this + 0x40),param_1);
  FUN_005f4d90(param_1,(int)this + 0x9c);
  iVar2 = (int)this + 0x6c;
  iVar3 = 10;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_005f4d90(param_1,(int)this + 0x94);
  FUN_005f4d90(param_1,(int)this + 0x98);
  iVar2 = (int)this + 0xa0;
  iVar3 = 4;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_005f4d90(param_1,(int)this + 0xbc);
  FUN_005f4d90(param_1,(int)this + 0xc0);
  FUN_005f4d90(param_1,(int)this + 0xc4);
  FUN_005f5350((void *)((int)this + 0xb0),param_1);
  iVar2 = (int)this + 200;
  iVar3 = 3;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_005f4d90(param_1,(int)this + 0xe0);
  FUN_005f4d90(param_1,(int)this + 0xe4);
  FUN_005f5350((void *)((int)this + 0xd4),param_1);
  FUN_005f4d90(param_1,(int)this + 0x68);
  (**(code **)(*(int *)((int)this + 0x24) + 0x10))(param_1);
  (**(code **)(*(int *)((int)this + 0x2c) + 0x10))(param_1);
  (**(code **)(*(int *)((int)this + 0x48) + 8))(param_1);
  FUN_004ece90((void *)((int)this + 0x38),param_1);
  FUN_004ece90((void *)((int)this + 0x3c),param_1);
  FUN_004ece90((void *)((int)this + 0x34),param_1);
  FUN_005f4d90(param_1,&stack0xfffffff8);
  if (unaff_EBP != 0) {
    piVar1 = FUN_00477630(this,unaff_EBP);
    *(int **)((int)this + 0x5c) = piVar1;
    if (piVar1 != (int *)0x0) {
      (**(code **)(*piVar1 + 4))(param_1);
    }
  }
  return;
}

