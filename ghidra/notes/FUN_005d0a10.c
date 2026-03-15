
void __thiscall FUN_005d0a10(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  
  FUN_005cee20(this,param_1);
  FUN_005f4db0(param_1,(int)this + 0x2c);
  FUN_005f4db0(param_1,(int)this + 0x30);
  FUN_005f4db0(param_1,(int)this + 0x34);
  iVar2 = (int)this + 0x5c;
  iVar1 = 9;
  do {
    FUN_005f4db0(param_1,iVar2 + -0x24);
    FUN_005f4db0(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  FUN_005f4db0(param_1,(int)this + 0x80);
  FUN_005f4db0(param_1,&DAT_006bc978);
  return;
}

