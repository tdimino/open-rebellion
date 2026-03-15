
uint __thiscall FUN_0051c360(void *this,int *param_1)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  uint uVar4;
  
  piVar2 = param_1;
  *param_1 = 0;
  uVar4 = *(uint *)((int)this + 4);
  if (uVar4 != 0) {
    piVar3 = *(int **)((int)this + 8);
    iVar1 = *(int *)(*piVar3 + 4);
    if (*(int *)((int)piVar3 + iVar1 + 0x34) < 0) {
      FUN_00615ee0((LPCRITICAL_SECTION)((int)piVar3 + iVar1 + 0x38));
    }
    *(undefined4 *)((int)piVar3 + iVar1 + 8) = 0;
    if (*(int *)((int)piVar3 + iVar1 + 0x34) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)piVar3 + iVar1 + 0x38));
    }
    FUN_00616c30(*(void **)((int)this + 8),0);
    FUN_005f4ce0(*(void **)((int)this + 8),&param_1);
    piVar3 = (int *)FUN_0051f8f0((int)param_1);
    uVar4 = (uint)(piVar3 != (int *)0x0);
    *piVar2 = (int)piVar3;
    if ((uVar4 != 0) && ((**(code **)(*piVar3 + 4))(*(undefined4 *)((int)this + 8)), uVar4 != 0)) {
      return uVar4;
    }
  }
  if ((undefined4 *)*piVar2 != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)*piVar2)(1);
    *piVar2 = 0;
  }
  return uVar4;
}

