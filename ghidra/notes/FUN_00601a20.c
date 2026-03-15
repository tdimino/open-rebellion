
void __thiscall FUN_00601a20(void *this,int *param_1)

{
  int iVar1;
  char *pcVar2;
  
  FUN_00615f00(param_1 + 3,0);
  pcVar2 = (char *)FUN_00615680(*(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1));
  FUN_00601aa0(this,pcVar2);
  FUN_00615660(*(void **)(*(int *)(*param_1 + 4) + 4 + (int)param_1),0);
  FUN_00616900(param_1 + 3,0);
  iVar1 = *(int *)(*param_1 + 4);
  if (*(int *)((int)param_1 + iVar1 + 0x34) < 0) {
    FUN_00615ee0((LPCRITICAL_SECTION)((int)param_1 + iVar1 + 0x38));
  }
  *(undefined4 *)((int)param_1 + iVar1 + 8) = 0;
  if (*(int *)((int)param_1 + iVar1 + 0x34) < 0) {
    FUN_00615ef0((LPCRITICAL_SECTION)((int)param_1 + iVar1 + 0x38));
  }
  return;
}

