
int __thiscall FUN_0051ad90(void *this,void *param_1)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  int local_c;
  int local_8;
  uint local_4;
  
  piVar1 = (int *)FUN_00505f60((uint *)((int)this + 0x154));
  local_4 = 0;
  local_8 = 0;
  local_c = 0;
  iVar2 = FUN_0055d550(&local_4,&local_8,&local_c);
  if (piVar1 != (int *)0x0) {
    iVar3 = FUN_0051be90(piVar1,local_4,local_8,local_c,param_1);
    if ((iVar3 != 0) && (iVar2 != 0)) {
      return 1;
    }
    iVar2 = 0;
  }
  return iVar2;
}

