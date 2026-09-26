
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005eae90(void *this,undefined4 *param_1)

{
  float fVar1;
  int iVar2;
  void *this_00;
  undefined4 *puVar3;
  undefined4 local_10 [4];
  
  if (((*(float *)((int)this + 0x68) == DAT_006bc940) &&
      (*(float *)((int)this + 0x6c) == DAT_006bc944)) &&
     (*(float *)((int)this + 0x70) == DAT_006bc948)) {
    *(undefined4 *)((int)this + 0x2c) = *(undefined4 *)((int)this + 0x1c);
    *(undefined4 *)((int)this + 0x30) = *(undefined4 *)((int)this + 0x20);
    *(undefined4 *)((int)this + 0x34) = *(undefined4 *)((int)this + 0x24);
    *(undefined4 *)((int)this + 0x38) = *(undefined4 *)((int)this + 0x28);
  }
  else {
    *(undefined4 *)((int)this + 0x2c) = *(undefined4 *)((int)this + 0x68);
    *(undefined4 *)((int)this + 0x30) = *(undefined4 *)((int)this + 0x6c);
    *(undefined4 *)((int)this + 0x34) = *(undefined4 *)((int)this + 0x70);
    *(undefined4 *)((int)this + 0x38) = *(undefined4 *)((int)this + 0x74);
  }
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x38) = 0xbf800000;
  if (((*(float *)((int)this + 0x2c) == DAT_006bc940) &&
      (*(float *)((int)this + 0x30) == DAT_006bc944)) &&
     (*(float *)((int)this + 0x34) == DAT_006bc948)) {
    iVar2 = FUN_005c6070(this);
    if (iVar2 == 0) {
      *(undefined4 *)((int)this + 0x2c) = 0;
      *(undefined4 *)((int)this + 0x30) = 0;
      *(undefined4 *)((int)this + 0x34) = 0x3f800000;
      *(undefined4 *)((int)this + 0x38) = 0xbf800000;
    }
    else {
      iVar2 = *(int *)(iVar2 + 0x168);
      puVar3 = local_10;
      this_00 = (void *)FUN_005aaf90();
      puVar3 = (undefined4 *)FUN_005a7490(this_00,puVar3,iVar2);
      *(undefined4 *)((int)this + 0x2c) = *puVar3;
      *(undefined4 *)((int)this + 0x30) = puVar3[1];
      *(undefined4 *)((int)this + 0x34) = puVar3[2];
      *(undefined4 *)((int)this + 0x38) = puVar3[3];
    }
  }
  if (*(int *)((int)this + 0x38) != 0x3f800000) {
    if (*(int *)((int)this + 0x38) == -0x40800000) {
      *(float *)((int)this + 0x38) =
           SQRT(*(float *)((int)this + 0x2c) * *(float *)((int)this + 0x2c) +
                *(float *)((int)this + 0x34) * *(float *)((int)this + 0x34) +
                *(float *)((int)this + 0x30) * *(float *)((int)this + 0x30));
    }
    if (_DAT_0066d874 < *(float *)((int)this + 0x38)) {
      fVar1 = _DAT_0066d890 / *(float *)((int)this + 0x38);
      *(undefined4 *)((int)this + 0x38) = 0xbf800000;
      *(float *)((int)this + 0x2c) = *(float *)((int)this + 0x2c) * fVar1;
      *(float *)((int)this + 0x30) = fVar1 * *(float *)((int)this + 0x30);
      *(float *)((int)this + 0x34) = fVar1 * *(float *)((int)this + 0x34);
    }
    *(undefined4 *)((int)this + 0x38) = 0x3f800000;
  }
  *param_1 = *(undefined4 *)((int)this + 0x2c);
  param_1[1] = *(undefined4 *)((int)this + 0x30);
  param_1[2] = *(undefined4 *)((int)this + 0x34);
  param_1[3] = *(undefined4 *)((int)this + 0x38);
  return;
}

