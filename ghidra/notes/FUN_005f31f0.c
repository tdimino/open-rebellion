
void * __thiscall FUN_005f31f0(void *this,undefined4 param_1)

{
  undefined4 uVar1;
  
  FUN_00618ed0(&DAT_006bdcd8,(char *)&DAT_006ac1c4);
  uVar1 = FUN_005f36d0(*(char **)((int)this + 4),&DAT_006bdcd8,&param_1);
  *(undefined4 *)((int)this + 8) = uVar1;
  FUN_00618b60(*(undefined **)((int)this + 4));
  *(undefined4 *)((int)this + 4) = param_1;
  return this;
}

