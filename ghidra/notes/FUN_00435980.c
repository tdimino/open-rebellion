
int FUN_00435980(int *param_1,int *param_2,int param_3)

{
  int iVar1;
  int iVar2;
  void *this;
  int *piVar3;
  int iVar4;
  
  iVar4 = 0;
  this = (void *)FUN_0041c210(param_3);
  while ((this != (void *)0x0 && (iVar4 == 0))) {
    piVar3 = (int *)FUN_0041c2c0(this,&param_3);
    iVar1 = *piVar3;
    iVar2 = *param_1;
    FUN_00619730();
    if (iVar1 == iVar2) {
      iVar4 = 1;
      *param_2 = *param_2 + *(int *)((int)this + 0x14);
    }
    else {
      this = *(void **)((int)this + 8);
    }
  }
  return iVar4;
}

