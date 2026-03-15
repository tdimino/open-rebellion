
void __thiscall FUN_00402070(void *this,void *param_1,undefined4 param_2)

{
  uint *this_00;
  int iVar1;
  int iVar2;
  
  FUN_0058f0c0(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x1c);
  FUN_005f4d90(param_1,(int)this + 0x20);
  FUN_005f4d90(param_1,(int)this + 0x24);
  FUN_005f4d90(param_1,(int)this + 0x28);
  FUN_005f4d90(param_1,(int)this + 0x2c);
  this_00 = (uint *)((int)this + 0x30);
  FUN_005f4990(this_00,param_1);
  FUN_005f4990((void *)((int)this + 0x34),param_1);
  FUN_004ece90((void *)((int)this + 0x38),param_1);
  iVar1 = (int)this + 0x40;
  iVar2 = 10;
  do {
    FUN_005f4d90(param_1,iVar1);
    iVar1 = iVar1 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  *(undefined4 *)((int)this + 0x3c) = param_2;
  *this_00 = *this_00 | 0x80000000;
  return;
}

