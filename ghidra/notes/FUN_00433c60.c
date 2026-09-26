
void __thiscall FUN_00433c60(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  
  FUN_0058f0c0(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x1c);
  FUN_005f4d90(param_1,(int)this + 0x20);
  FUN_005f4d90(param_1,(undefined4 *)((int)this + 0x24));
  FUN_005f4d90(param_1,(int)this + 0x44);
  FUN_005f4d90(param_1,(int)this + 0x38);
  FUN_005f4dd0(param_1,(int)this + 0x34);
  FUN_005f4dd0(param_1,(int)this + 0x36);
  FUN_005f4990((void *)((int)this + 0x28),param_1);
  FUN_005f4990((void *)((int)this + 0x30),param_1);
  FUN_005f4990((void *)((int)this + 0x2c),param_1);
  iVar2 = (int)this + 0x48;
  iVar1 = 0x3c;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  *(undefined4 *)((int)this + 0x24) = 1;
  return;
}

