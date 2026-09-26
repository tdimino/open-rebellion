
void __thiscall FUN_004ebcc0(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  
  FUN_0058f0c0(this,param_1);
  FUN_005f4d90(param_1,(undefined4 *)((int)this + 0x1c));
  FUN_005f4d90(param_1,(int)this + 0x20);
  FUN_005f4d90(param_1,(int)this + 0x38);
  FUN_005f4d90(param_1,(int)this + 0x3c);
  FUN_005f4990((void *)((int)this + 0x24),param_1);
  FUN_005f4990((void *)((int)this + 0x28),param_1);
  FUN_005f4dd0(param_1,(int)this + 0x2c);
  FUN_005f4dd0(param_1,(int)this + 0x2e);
  iVar1 = (int)this + 0x40;
  iVar2 = 0x16;
  do {
    FUN_005f4d90(param_1,iVar1);
    iVar1 = iVar1 + 4;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  iVar2 = (int)this + 0x98;
  iVar1 = 1;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  *(undefined4 *)((int)this + 0x1c) = 1;
  return;
}

