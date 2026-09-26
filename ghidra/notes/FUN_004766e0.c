
void __thiscall FUN_004766e0(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  
  FUN_005f5630(this,param_1);
  FUN_005f4db0(param_1,(int)this + 0x1c);
  FUN_005f4db0(param_1,(int)this + 0x20);
  FUN_005f4db0(param_1,(int)this + 0x44);
  FUN_005f49a0((void *)((int)this + 0x60),param_1);
  FUN_005f49a0((void *)((int)this + 100),param_1);
  FUN_004ecea0((void *)((int)this + 0x40),param_1);
  FUN_005f4db0(param_1,(int)this + 0x9c);
  iVar1 = (int)this + 0x6c;
  iVar2 = 10;
  do {
    FUN_005f4db0(param_1,iVar1);
    iVar1 = iVar1 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  FUN_005f4db0(param_1,(int)this + 0x94);
  FUN_005f4db0(param_1,(int)this + 0x98);
  iVar1 = (int)this + 0xa0;
  iVar2 = 4;
  do {
    FUN_005f4db0(param_1,iVar1);
    iVar1 = iVar1 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  FUN_005f4db0(param_1,(int)this + 0xbc);
  FUN_005f4db0(param_1,(int)this + 0xc0);
  FUN_005f4db0(param_1,(int)this + 0xc4);
  FUN_005f53f0((void *)((int)this + 0xb0),param_1);
  iVar1 = (int)this + 200;
  iVar2 = 3;
  do {
    FUN_005f4db0(param_1,iVar1);
    iVar1 = iVar1 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  FUN_005f4db0(param_1,(int)this + 0xe0);
  FUN_005f4db0(param_1,(int)this + 0xe4);
  FUN_005f53f0((void *)((int)this + 0xd4),param_1);
  FUN_005f4db0(param_1,(int)this + 0x68);
  (**(code **)(*(int *)((int)this + 0x24) + 0x18))(param_1);
  (**(code **)(*(int *)((int)this + 0x2c) + 0x18))(param_1);
  (**(code **)(*(int *)((int)this + 0x48) + 0xc))(param_1);
  FUN_004ecea0((void *)((int)this + 0x38),param_1);
  FUN_004ecea0((void *)((int)this + 0x3c),param_1);
  FUN_004ecea0((void *)((int)this + 0x34),param_1);
  if (*(int **)((int)this + 0x5c) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0x5c) + 0x14))();
    FUN_005f4db0(param_1,&stack0xfffffff8);
    (**(code **)(**(int **)((int)this + 0x5c) + 8))(param_1);
    return;
  }
  FUN_005f4db0(param_1,&DAT_0065a3ac);
  return;
}

