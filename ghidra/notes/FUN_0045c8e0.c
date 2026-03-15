
int * __thiscall FUN_0045c8e0(void *this,ushort *param_1,undefined4 param_2)

{
  int iVar1;
  void *pvVar2;
  int *piVar3;
  
  switch(param_2) {
  case 4:
    iVar1 = 0x10;
    break;
  default:
    goto switchD_0045c8ef_caseD_5;
  case 9:
    iVar1 = 4;
    break;
  case 10:
    iVar1 = 8;
    break;
  case 0xb:
    iVar1 = 0x40;
  }
  pvVar2 = (void *)FUN_0060a860((void *)((int)this + 0x174),iVar1 << 0x10 | (uint)*param_1);
  if (pvVar2 != (void *)0x0) {
    piVar3 = FUN_0045aac0(this,pvVar2,*(int *)((int)pvVar2 + 0x40),*(uint *)((int)pvVar2 + 0x44));
    return piVar3;
  }
switchD_0045c8ef_caseD_5:
  return (int *)0x0;
}

