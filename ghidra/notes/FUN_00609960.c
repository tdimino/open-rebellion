
void __thiscall FUN_00609960(void *this,int param_1)

{
  undefined1 uVar1;
  int iVar2;
  int iVar3;
  void *this_00;
  undefined2 local_c;
  short local_4;
  short local_2;
  
  iVar2 = *(int *)(param_1 + 0x2c);
  iVar3 = *(int *)(param_1 + 0x28);
  uVar1 = *(undefined1 *)(*(int *)((int)this + 0x110) + 8);
  this_00 = (void *)FUN_0060bd00(param_1);
  if (this_00 != (void *)0x0) {
    FUN_005fd0f0(this_00,*(uint **)((int)this + 0xa4),iVar3,iVar2);
    local_c = (undefined2)iVar2;
    FUN_00609dd0(this,(undefined4 *)&local_4,CONCAT22(local_c,(short)iVar3));
    if ((*(byte *)(param_1 + 0x3c) & 1) != 0) {
      FUN_005fd0f0(this_00,*(uint **)((int)this + 0x110),(int)local_4,(int)local_2);
      *(undefined1 *)(*(int *)((int)this + 0x110) + 8) = uVar1;
    }
  }
  return;
}

