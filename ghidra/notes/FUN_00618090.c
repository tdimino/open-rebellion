
int __thiscall FUN_00618090(void *this,char param_1)

{
  undefined4 *puVar1;
  char *pcVar2;
  int iVar3;
  int unaff_ESI;
  
  if (*(uint *)((int)this + 0x24) < *(uint *)((int)this + 0x28)) {
    pcVar2 = (char *)(*(uint *)((int)this + 0x28) - 1);
    *(char **)((int)this + 0x28) = pcVar2;
    *pcVar2 = param_1;
    return (int)param_1;
  }
                    /* WARNING: Load size is inaccurate */
  iVar3 = (**(code **)(*this + 0xc))(0xffffffff,1,1);
  if (iVar3 == -1) {
    return -1;
  }
  if ((*(int *)((int)this + 8) == 0) && (*(int *)((int)this + 0x2c) != 0)) {
    puVar1 = *(undefined4 **)((int)this + 0x28);
    FUN_0061be90((undefined4 *)((int)puVar1 + 1),puVar1,
                 (*(int *)((int)this + 0x2c) - (int)puVar1) - 1);
    **(undefined1 **)((int)this + 0x28) = (char)unaff_ESI;
  }
  return unaff_ESI;
}

