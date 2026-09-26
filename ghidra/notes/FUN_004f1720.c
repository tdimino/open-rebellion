
int __thiscall FUN_004f1720(void *this,undefined4 param_1,int param_2,void *param_3)

{
  void *pvVar1;
  int iVar2;
  uint *puVar3;
  int iVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063efd8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_004eed90(this,param_3);
  iVar4 = 0;
  if (iVar2 != 0) {
    iVar2 = FUN_004ef030(this,pvVar1);
    iVar4 = 0;
    if (iVar2 != 0) {
      iVar2 = FUN_004eeed0(this,pvVar1);
      iVar4 = 0;
      if (iVar2 != 0) {
        if ((param_2 != 0) && ((*(byte *)((int)this + 0x78) & 0x10) != 0)) {
          puVar3 = FUN_0042d170(this,&param_3);
          local_4 = 0;
          iVar4 = FUN_004ece60(puVar3);
          local_4 = 0xffffffff;
          FUN_00619730();
          if (iVar4 != 0) {
            iVar2 = FUN_00534640(this,1,pvVar1);
          }
        }
        iVar4 = 0;
        if (iVar2 != 0) {
          iVar4 = FUN_004f7800(this,(uint)(param_2 != 0),pvVar1);
        }
      }
    }
  }
  ExceptionList = local_c;
  return iVar4;
}

